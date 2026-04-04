use std::fmt::format;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
impl TokenType {
    pub fn xml_tag(&self)->&'static str{
        match self {
            TokenType::NUMBER => "integerConstant",
            TokenType::STRING => "stringConstant",
            TokenType::IDENT => "identifier",

            TokenType::CLASS | TokenType::CONSTRUCTOR |
            TokenType::FUNCTION | TokenType::METHOD |
            TokenType::FIELD | TokenType::STATIC |
            TokenType::VAR | TokenType::INT | TokenType::CHAR |
            TokenType::BOOLEAN | TokenType::VOID | TokenType::TRUE |
            TokenType::FALSE | TokenType::NULL | TokenType::THIS |
            TokenType::LET | TokenType::DO |TokenType::IF |
            TokenType::ELSE | TokenType::WHILE |
            TokenType::RETURN => "keyword",

            TokenType::LPAREN | TokenType::RPAREN | TokenType::LBRACE |
            TokenType::RBRACE | TokenType::LBRACKET | TokenType::RBRACKET |
            TokenType::COMMA | TokenType::SEMICOLON | TokenType::DOT |
            TokenType::PLUS | TokenType::MINUS | TokenType::ASTERISK |
            TokenType::SLASH | TokenType::AND | TokenType::OR | TokenType::NOT |
            TokenType::LT | TokenType::GT | TokenType::EQ => "symbol",

            TokenType::EOF => ""
            

            
        }
    }
}

pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: u32,


}
impl Token {
    pub fn new(kind:TokenType, lexeme:String, line:u32) -> Self{
        
        Self {
            kind,
            lexeme,
            line,
  
        }

    }
    pub fn to_xml(&self) -> String{
        if self.kind == TokenType::EOF {
            return "".to_string();
        }

        return format!("<{}> {} </{}>",self.kind.xml_tag(), self.lexeme, self.kind.xml_tag());

    }    
}