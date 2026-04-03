use crate::token::{Token, TokenType};
use std::collections::HashMap; 

struct Scanner {
    code: String,
    current: u32,
    line: u32,
    tokens: Vec<Token>,

    symbols: HashMap<char, TokenType>,
    
    keywords: HashMap<&'static str, TokenType>

}
impl Scanner{
    fn new(code:String) -> Self{
        //declaração de symbols da linguagem Jack
        let mut symbols = HashMap::new();
        symbols.insert('(', TokenType::LPAREN);
        symbols.insert(')', TokenType::RPAREN);

        symbols.insert('{', TokenType::LBRACE);
        symbols.insert('}', TokenType::RBRACE);

        symbols.insert('[', TokenType::LBRACKET);
        symbols.insert(']', TokenType::RBRACKET);

        symbols.insert(',', TokenType::COMMA);
        symbols.insert(';', TokenType::SEMICOLON);

        symbols.insert('.', TokenType::DOT);

        symbols.insert('+', TokenType::PLUS);
        symbols.insert('-', TokenType::MINUS);
        symbols.insert('*', TokenType::ASTERISK);
        symbols.insert('/', TokenType::SLASH);

        symbols.insert('&', TokenType::AND);
        symbols.insert('|', TokenType::OR);
        symbols.insert('~', TokenType::NOT);
        symbols.insert('<', TokenType::LT);
        symbols.insert('>', TokenType::GT);
        symbols.insert('=', TokenType::EQ);

        let mut keywords= HashMap::new();
        
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("constructor", TokenType::CONSTRUCTOR);
        keywords.insert("function", TokenType::FUNCTION);
        keywords.insert("method", TokenType::METHOD);
        keywords.insert("field", TokenType::FIELD);
        keywords.insert("static", TokenType::STATIC);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("int", TokenType::INT);
        keywords.insert("char", TokenType::CHAR);
        keywords.insert("boolean", TokenType::BOOLEAN);
        keywords.insert("void", TokenType::VOID);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("null", TokenType::NULL);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("let", TokenType::LET);
        keywords.insert("do", TokenType::DO);
        keywords.insert("if", TokenType::IF);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("while", TokenType::WHILE);
        keywords.insert("return", TokenType::RETURN);
        
        Self {
            code,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            symbols,
            keywords
        }






    }
}