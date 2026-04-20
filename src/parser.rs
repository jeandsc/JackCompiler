use std::array;

use crate::token::{Token, TokenType};

#[derive(Debug)]
enum ParserError {
    UnexpectedEOF,
    ExpectedToken(TokenType),
    // outros...
}

// Implemente Display e Error se quiser usar ? com anyhow ou std::error
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ParserError {}

pub struct Parser {
    tokens: Vec<Token>,
    pos:usize,
    xml_output: Vec<String>,
    ident_level:usize,

}
impl Parser {
    pub fn new(tokens:Vec<Token>) -> Self{
        Self { 
            tokens, 
            pos: 0, 
            xml_output:  Vec::new(),
            ident_level:0}
        }
    fn peek(&self, offset:usize) -> Option<Token>{
        if self.pos+offset < self.tokens.len(){
            return self.tokens.get(self.pos+offset).cloned();
        } else {
            return None;
        }
    }
    fn advance(&mut self){
        self.pos = self.pos+1;
    }
    fn open_tag(&mut self, tag_name:&str){
        let ident = "  ".repeat(self.ident_level);
        let tag = format!("{}<{}>", ident,tag_name);
        self.xml_output.push(tag);
        self.ident_level +=1;     
    }
    fn close_tag(&mut self, tag_name:&str){
        self.ident_level -=1;  
        let ident = "  ".repeat(self.ident_level);
        let tag = format!("{}</{}>", ident,tag_name);
        self.xml_output.push(tag);      
    }
    fn write_tag(&mut self, token:&Token){
        if token.kind == TokenType::EOF {
            return;
        }
        let ident = "  ".repeat(self.ident_level);
        let tag = format!("{}{}", ident, token.to_xml());
        self.xml_output.push(tag);
    }
    
    fn assert(&mut self, expected_type:TokenType){
        if let Some(token) = self.peek(0) {
            if token.kind == expected_type {
                self.write_tag(&token);
                self.advance();
            } else {
                panic!("SyntaxError: Esperava {} e encontrou {} na linha {}", expected_type, token.kind, token.line)
            }

        }
    }
    pub fn get_xml(&mut self)->String{
            self.xml_output.join("\n")
    }
    pub fn parse_code(&mut self){
        self.parse_class();
    }
    fn parse_class(&mut self){
        self.open_tag("class");
        self.assert(TokenType::CLASS);
        self.assert(TokenType::IDENT);
        self.assert(TokenType::LBRACE);

        self.assert(TokenType::RBRACE);
        self.assert(TokenType::EOF);
        self.close_tag("class");
    }
    fn parse_class_var_dec(&mut self) ->Result<(), ParserError>{
        self.open_tag("classVarDec");
        let modifiers = [TokenType::STATIC, TokenType::FIELD];
        let actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        if modifiers.contains(&actual.kind) {
        } else {
            if actual.kind == TokenType::STATIC {
                self.assert(TokenType::STATIC);
                //self.parse_type()
            }
        }
        self.close_tag("classVarDec");
        Ok(())   
    }
}