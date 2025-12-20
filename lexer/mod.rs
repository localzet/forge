mod token;
pub use token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            position: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.next_token();
            if let Some(t) = token {
                tokens.push(t);
            }
        }

        tokens.push(Token {
            token_type: TokenType::EndOfFile,
            lexeme: String::new(),
            line: self.line,
        });

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            ' ' | '\t' | '\r' => None,
            '\n' => {
                self.line += 1;
                None
            }

            '(' => Some(self.simple(TokenType::LeftParen)),
            ')' => Some(self.simple(TokenType::RightParen)),
            '{' => Some(self.simple(TokenType::LeftBrace)),
            '}' => Some(self.simple(TokenType::RightBrace)),
            ':' => {
                if self.match_char(':') {
                    Some(self.simple(TokenType::DoubleColon))
                } else {
                    Some(self.simple(TokenType::Colon))
                }
            }
            ';' => Some(self.simple(TokenType::Semicolon)),
            ',' => Some(self.simple(TokenType::Comma)),
            '.' => Some(self.simple(TokenType::Dot)),
            '[' => Some(self.simple(TokenType::LeftSquare)),
            ']' => Some(self.simple(TokenType::RightSquare)),
            '+' => Some(self.simple(TokenType::Plus)),
            '-' => Some(self.simple(TokenType::Minus)),
            '*' => Some(self.simple(TokenType::Star)),

            '=' => {
                if self.match_char('=') {
                    Some(self.simple(TokenType::EqualEqual))
                } else if self.match_char('>') {
                    Some(self.simple(TokenType::Arrow))
                } else {
                    Some(self.simple(TokenType::Equal))
                }
            }

            '"' => Some(self.string()),

            d if d.is_ascii_digit() => Some(self.number(d)),
            a if is_identifier_start(a) => Some(self.identifier(a)),

            '/' => Some(self.simple(TokenType::Slash)),

            _ => panic!("Unexpected character '{}' at line {}", c, self.line),
        }
    }

    fn simple(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: String::new(),
            line: self.line,
        }
    }

    fn string(&mut self) -> Token {
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            value.push(self.advance());
        }

        self.advance(); // closing "

        Token {
            token_type: TokenType::String,
            lexeme: value,
            line: self.line,
        }
    }

    fn number(&mut self, first: char) -> Token {
        let mut value = String::new();
        value.push(first);

        while self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        Token {
            token_type: TokenType::Integer,
            lexeme: value,
            line: self.line,
        }
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut value = String::new();
        value.push(first);

        while is_identifier_part(self.peek()) {
            value.push(self.advance());
        }

        let token_type = match value.as_str() {
            "let" => TokenType::Let,
            "function" => TokenType::Function,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "structure" => TokenType::Structure,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "enum" => TokenType::Enum,
            "trait" => TokenType::Trait,
            _ => TokenType::Identifier,
        };

        Token {
            token_type,
            lexeme: value,
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let c = self.source[self.position];
            self.position += 1;
            c
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.position] != expected {
            return false;
        }
        self.position += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_part(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit()
}
