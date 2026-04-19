
use std::fmt;
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
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::NUMBER => "number",
            TokenType::STRING => "string",
            TokenType::IDENT => "identifier",
            
          
            TokenType::CLASS => "class",
            TokenType::CONSTRUCTOR => "constructor",
            TokenType::FUNCTION => "function",
            TokenType::METHOD => "method",
            TokenType::FIELD => "field",
            TokenType::STATIC => "static",
            TokenType::VAR => "var",
            TokenType::INT => "int",
            TokenType::CHAR => "char",
            TokenType::BOOLEAN => "boolean",
            TokenType::VOID => "void",
            TokenType::TRUE => "true",
            TokenType::FALSE => "false",
            TokenType::NULL => "null",
            TokenType::THIS => "this",
            TokenType::LET => "let",
            TokenType::DO => "do",
            TokenType::IF => "if",
            TokenType::ELSE => "else",
            TokenType::WHILE => "while",
            TokenType::RETURN => "return",
            
    
            TokenType::LPAREN => "(",
            TokenType::RPAREN => ")",
            TokenType::LBRACE => "{",
            TokenType::RBRACE => "}",
            TokenType::LBRACKET => "[",
            TokenType::RBRACKET => "]",
            TokenType::COMMA => ",",
            TokenType::SEMICOLON => ";",
            TokenType::DOT => ".",
            TokenType::PLUS => "+",
            TokenType::MINUS => "-",
            TokenType::ASTERISK => "*",
            TokenType::SLASH => "/",
            TokenType::AND => "&",
            TokenType::OR => "|",
            TokenType::NOT => "~",
            TokenType::LT => "<",
            TokenType::GT => ">",
            TokenType::EQ => "=",
            
            TokenType::EOF => "EOF",
  

        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
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
        if self.kind == TokenType::AND {
            return format!("<{}> &amp; </{}>",self.kind.xml_tag(), self.kind.xml_tag());
        } else if self.kind == TokenType::GT {
            return format!("<{}> &gt; </{}>",self.kind.xml_tag(), self.kind.xml_tag());
        } else if self.kind == TokenType::LT {
            return format!("<{}> &lt; </{}>",self.kind.xml_tag(), self.kind.xml_tag());
        } else if self.kind == TokenType::EOF {
            return "".to_string();
        } else {
            return format!("<{}> {} </{}>",self.kind.xml_tag(), self.lexeme, self.kind.xml_tag());
        }

        

    }    
}