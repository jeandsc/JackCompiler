use crate::token::{Token, TokenType};

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
        let ident = " ".repeat(self.ident_level);
        let tag = format!("{}<{}>", ident,tag_name);
        self.xml_output.push(tag);      
    }
    fn close_tag(&mut self, tag_name:&str){
        let ident = " ".repeat(self.ident_level);
        let tag = format!("{}</{}>", ident,tag_name);
        self.xml_output.push(tag);      
    }
    fn write_tag(&mut self, token:&Token){
        let ident = " ".repeat(self.ident_level);
        let tag = format!("{}{}", ident, token.to_xml());
        self.xml_output.push(tag);
    }
    
    fn assert(&mut self, expected_type:TokenType){
        if let Some(token) = self.peek(0) {
            if token.kind == expected_type {
                self.write_tag(&token);
            } else {
                //panic!("SyntaxError: Esperava {} e encontrou {} na linha {}", expected_type, token.kind, token.line)
            }

        }
    }
}