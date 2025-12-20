use crate::ast::expression::*;
use crate::lexer::token::*;
use crate::lexer::Lexer;

pub struct ExpressionParser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse_expression(&mut self) -> Expression {
        match self.current().token_type {
            TokenType::Integer => {
                let value = self.current().lexeme.parse::<i64>().unwrap();
                self.advance();
                Expression::IntegerLiteral(value)
            }
            TokenType::String => {
                let value = self.current().lexeme.clone();
                self.advance();
                Expression::StringLiteral(value)
            }
            TokenType::True => {
                self.advance();
                Expression::BooleanLiteral(true)
            }
            TokenType::False => {
                self.advance();
                Expression::BooleanLiteral(false)
            }
            TokenType::Identifier => {
                let name = self.current().lexeme.clone();
                self.advance();
                Expression::Variable(name)
            }
            TokenType::LeftBracket => self.parse_array_literal(),
            _ => panic!(
                "Unexpected token {:?} at expression",
                self.current().token_type
            ),
        }
    }

    fn parse_array_literal(&mut self) -> Expression {
        self.advance(); // skip '['
        let mut elements = Vec::new();

        while self.current().token_type != TokenType::RightBracket {
            let key = if self.current().token_type == TokenType::String
                || self.current().token_type == TokenType::Integer
            {
                let k = self.current().lexeme.clone();
                self.advance();
                if self.current().token_type == TokenType::Arrow {
                    self.advance(); // skip =>
                    Some(k)
                } else {
                    None
                }
            } else {
                None
            };

            let value = self.parse_expression();
            elements.push((key, value));

            if self.current().token_type == TokenType::Comma {
                self.advance();
            } else {
                break;
            }
        }

        if self.current().token_type != TokenType::RightBracket {
            panic!("Expected ']' in array literal");
        }
        self.advance(); // skip ']'

        Expression::ArrayLiteral(elements)
    }
}
