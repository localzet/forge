use crate::ast::*;
use crate::lexer::token::*;
use crate::lexer::Lexer;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
    }

    fn expect(&mut self, tt: TokenType, msg: &str) {
        if self.current().token_type != tt {
            panic!("{} at line {} (found: {:?})", msg, self.current().line, self.current().token_type);
        }
        self.advance();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut functions = Vec::new();

        while self.current().token_type != TokenType::EndOfFile {
            if self.current().token_type == TokenType::Function {
                functions.push(self.parse_function());
            } else {
                panic!("Top-level only supports function declarations. Found {:?} at line {}", self.current().token_type, self.current().line);
            }
        }

        Program { functions }
    }

    fn parse_function(&mut self) -> Function {
        self.expect(TokenType::Function, "Expected 'function'");
        let name = if self.current().token_type == TokenType::Identifier {
            let n = self.current().lexeme.clone();
            self.advance();
            n
        } else {
            panic!("Expected function name")
        };

        self.expect(TokenType::LeftParen, "Expected '('");
        let mut params = Vec::new();
        while self.current().token_type != TokenType::RightParen {
            // param: name [: type]
            if self.current().token_type != TokenType::Identifier {
                panic!("Expected parameter name");
            }
            let pname = self.current().lexeme.clone();
            self.advance();
            let ptype = if self.current().token_type == TokenType::Colon {
                self.advance();
                if self.current().token_type != TokenType::Identifier {
                    panic!("Expected type name after ':'");
                }
                let t = Some(self.current().lexeme.clone());
                self.advance();
                t
            } else {
                None
            };

            params.push((pname, ptype));

            if self.current().token_type == TokenType::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenType::RightParen, "Expected ')'");

        let return_type = if self.current().token_type == TokenType::Colon {
            self.advance();
            if self.current().token_type != TokenType::Identifier {
                panic!("Expected return type identifier");
            }
            let rt = Some(self.current().lexeme.clone());
            self.advance();
            rt
        } else {
            None
        };

        self.expect(TokenType::LeftBrace, "Expected '{' for function body");
        let mut body = Vec::new();
        while self.current().token_type != TokenType::RightBrace {
            body.push(self.parse_statement());
        }
        self.expect(TokenType::RightBrace, "Expected '}' after function body");

        Function { name, parameters: params, return_type, body }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current().token_type {
            TokenType::Let => self.parse_let(),
            TokenType::Return => self.parse_return(),
            _ => {
                let expr = self.parse_expression();
                self.expect(TokenType::Semicolon, "Expected ';' after expression");
                Statement::Expression(expr)
            }
        }
    }

    fn parse_let(&mut self) -> Statement {
        self.expect(TokenType::Let, "Expected 'let'");
        if self.current().token_type != TokenType::Identifier {
            panic!("Expected identifier after let");
        }
        let name = self.current().lexeme.clone();
        self.advance();
        let type_name = if self.current().token_type == TokenType::Colon {
            self.advance();
            if self.current().token_type != TokenType::Identifier {
                panic!("Expected type after ':' in let");
            }
            let t = Some(self.current().lexeme.clone());
            self.advance();
            t
        } else {
            None
        };
        self.expect(TokenType::Equal, "Expected '=' in let statement");
        let value = self.parse_expression();
        self.expect(TokenType::Semicolon, "Expected ';' after let");
        Statement::Let { name, type_name, value }
    }

    fn parse_return(&mut self) -> Statement {
        self.expect(TokenType::Return, "Expected 'return'");
        if self.current().token_type == TokenType::Semicolon {
            self.advance();
            Statement::Return(None)
        } else {
            let expr = self.parse_expression();
            self.expect(TokenType::Semicolon, "Expected ';' after return");
            Statement::Return(Some(expr))
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expression {
        match &self.current().token_type {
            TokenType::Integer => {
                let v = self.current().lexeme.parse::<i64>().unwrap();
                self.advance();
                Expression::IntegerLiteral(v)
            }
            TokenType::String => {
                let s = self.current().lexeme.clone();
                self.advance();
                Expression::StringLiteral(s)
            }
            TokenType::True => { self.advance(); Expression::BooleanLiteral(true) }
            TokenType::False => { self.advance(); Expression::BooleanLiteral(false) }
            TokenType::Identifier => {
                let name = self.current().lexeme.clone();
                self.advance();
                // function call
                if self.current().token_type == TokenType::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current().token_type != TokenType::RightParen {
                        args.push(self.parse_expression());
                        if self.current().token_type == TokenType::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(TokenType::RightParen, "Expected ')'");
                    Expression::FunctionCall(name, args)
                } else if self.current().token_type == TokenType::LeftSquare {
                    // array access like a[0]
                    self.advance();
                    let idx = self.parse_expression();
                    self.expect(TokenType::RightSquare, "Expected ']'");
                    Expression::ArrayAccess(Box::new(Expression::Variable(name)), Box::new(idx))
                } else {
                    Expression::Variable(name)
                }
            }
            TokenType::LeftSquare => self.parse_array_literal(),
            TokenType::LeftParen => {
                self.advance();
                let e = self.parse_expression();
                self.expect(TokenType::RightParen, "Expected ')'");
                e
            }
            other => panic!("Unexpected token in expression: {:?} at line {}", other, self.current().line),
        }
    }

    fn parse_array_literal(&mut self) -> Expression {
        self.expect(TokenType::LeftSquare, "Expected '['");
        let mut elements = Vec::new();
        while self.current().token_type != TokenType::RightSquare {
            // Peek to see if this element is a key => value pair
            let key = if (self.current().token_type == TokenType::String || self.current().token_type == TokenType::Integer)
                && self.tokens.get(self.position + 1).map_or(false, |t| t.token_type == TokenType::Arrow)
            {
                let k = self.current().lexeme.clone();
                // consume key and arrow
                self.advance();
                self.advance();
                Some(k)
            } else {
                None
            };

            let value = self.parse_expression();
            elements.push((key, Box::new(value)));

            if self.current().token_type == TokenType::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(TokenType::RightSquare, "Expected ']'");
        Expression::ArrayLiteral(elements)
    }
}
