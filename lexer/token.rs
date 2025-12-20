#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // literals
    Integer,
    String,
    Boolean,

    // identifiers
    Identifier,
    
    // brackets
    LeftBracket,
    RightBracket,

    // keywords
    Let,
    Function,
    Return,
    If,
    Else,
    Structure,
    True,
    False,
    Enum,
    Trait,

    // symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Comma,
    Dot,

    // square brackets
    LeftSquare,
    RightSquare,
    Arrow,
    DoubleColon,

    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
