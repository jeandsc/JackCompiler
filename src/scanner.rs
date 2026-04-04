use crate::token::{Token, TokenType};
use std::collections::HashMap; 

pub struct Scanner {
    code: String,
    current: u32,
    line: u32,
    tokens: Vec<Token>,

    symbols: HashMap<char, TokenType>,
    
    keywords: HashMap<&'static str, TokenType>

}
impl Scanner{
    pub fn new(code:String) -> Self{
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

    fn peek(&self, offset:u32) -> char{
        let pos = self.current + offset;
        //está contando caracteres na string
        if pos < (self.code.chars().count()) as u32 {
            return self.code.chars().nth(pos as usize).unwrap();
        } else {
            return '\0'; 
        }
        
    }

    fn advance(&mut self) {
        let size = self.code.chars().count() as u32 ;
        if self.current < size {
            if self.code.chars().nth(self.current as usize).unwrap() == '\n' {
                self.line+=1;
            }
            self.current+=1;
        }
    }
    fn skip_whitespace(&mut self){
        loop {
            let c = self.peek(0);
            if c == ' ' || c == '\t' {
                self.advance();
            }
            else if c == '\r' {
                self.advance();
            }
            else if c == '\n'{
                self.advance();
            } else {
                break;
            }
        }
    }
    fn read_number(&mut self) -> Token{
        let start = self.current;
        while self.peek(0).is_numeric() {
            self.advance();
        }
        let lexeme = self.code.chars().skip(start as usize).take((self.current-start) as usize).collect();
        return Token::new(TokenType::NUMBER, lexeme, self.line);
    }
    fn read_string(&mut self) -> Token {
        
        self.advance();

        let start = self.current;
        
        while (self.peek(0) != '\"') && (self.peek(0) != '\0'){
            let mut c = self.peek(0);
            if c == '\n' {
                panic!("String não fechada em  {}", self.line);
            }
            
            self.advance();
            
        }
        
        if self.peek(0) == '\0'{
            panic!("String não fechada em  {}", self.line);
        }
        let lexeme = self.code.chars().skip(start as usize).take((self.current-start) as usize).collect();
        self.advance();

        
        return Token::new(TokenType::STRING, lexeme, self.line);

    }
    fn read_identifier_keyword(&mut self) ->Token {
        let start =self.current;
        self.advance();
        let mut ch = self.peek(0);
        while ch.is_ascii_alphanumeric() || ch == '_' {
            self.advance();
            ch = self.peek(0);
        }
        let lexeme:String = self.code.chars().skip(start as usize).take((self.current-start) as usize).collect();
        
        if let Some(v) = self.keywords.get(lexeme.as_str()) {
            return Token::new(*v, lexeme, self.line);
        } else {
            return Token::new(TokenType::IDENT, lexeme, self.line);
        }
        
        
    }
    fn read_symbol(&mut self) -> Token{
        let ch = self.peek(0);
        let lexeme = ch.to_string();
        if let Some(v) = self.symbols.get(&ch){
            let symb = Token::new(*v, lexeme, self.line);
            self.advance();
            return symb;
        } else {
            !panic!("SyntaxError: Simbolo Desconhecido")
        }
    }
    pub fn tokenize(&mut self) -> &Vec<Token> {
        let size = self.code.chars().count() as u32 ;
        while self.current < size {

            let ch = {
                self.skip_whitespace();
                self.peek(0)
            };
            if ch.is_ascii_digit(){
                let token = self.read_number();
                self.tokens.push(token);
            } else if ch == '\"'  {
                let token = self.read_string();
                self.tokens.push(token)
            } else if ch.is_ascii_alphabetic() || ch=='_'{
                let token = self.read_identifier_keyword();
                self.tokens.push(token);
            }else if let Some(v) = self.symbols.get(&ch){
                let token = self.read_symbol();
                self.tokens.push(token);
            }else {
                self.advance();
            }
        }
        let eof = Token::new(TokenType::EOF, "".to_string(), self.line);
        self.tokens.push(eof);

        return &self.tokens;
    }
    

}