#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {

    //literals

    NUMBER,
    STRING,
    IDENT,

    // Symbols
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COMMA,
    SEMICOLON,
    DOT,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    AND,
    OR,
    NOT,
    LT,
    GT,
    EQ,

    //Keywords
    CLASS,
    CONSTRUCTOR,
    FUNCTION,
    METHOD,
    FIELD,
    STATIC,
    VAR,
    INT,
    CHAR,
    BOOLEAN,
    VOID,
    TRUE,
    FALSE,
    NULL,
    THIS,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,

    EOF,


}

pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: u32,
}