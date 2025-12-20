use crate::ast::statement::*;
use crate::ast::expression::*;
use crate::lexer::token::*;

pub struct StatementParser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> StatementParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse_let(&mut self) -> Statement {
        self.advance(); // skip let
        let name = self.current().lexeme.clone();
        self.advance();
        let type_name = if self.current().token_type == TokenType::Colon {
            self.advance();
            Some(self.current().lexeme.clone())
        } else {
            None
        };
        self.advance(); // skip type
        if self.current().token_type != TokenType::Equal {
            panic!("Expected '=' in let statement");
        }
        self.advance(); // skip '='
        let expr_parser = super::expression::ExpressionParser::new(self.tokens);
        let value = expr_parser.parse_expression();
        if self.current().token_type != TokenType::Semicolon {
            panic!("Expected ';' at end of let");
        }
        self.advance();
        Statement::Let {
            name,
            type_name,
            value,
        }
    }
}
