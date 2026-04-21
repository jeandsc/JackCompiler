use std::array;

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum ParserError {
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
    fn parse_class(&mut self) ->Result<(), ParserError>{
        let modifiers = [TokenType::STATIC, TokenType::FIELD];
        self.open_tag("class");
        self.assert(TokenType::CLASS);
        self.assert(TokenType::IDENT);
        self.assert(TokenType::LBRACE);
        let mut actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        while modifiers.contains(&actual.kind){
            self.parse_class_var_dec();
            actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        }
        self.assert(TokenType::RBRACE);
        self.assert(TokenType::EOF);
        self.close_tag("class");
        Ok(())
    }
    fn parse_class_var_dec(&mut self) ->Result<(), ParserError>{
        self.open_tag("classVarDec");
        let modifiers = [TokenType::STATIC, TokenType::FIELD];
        let actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        if modifiers.contains(&actual.kind) {
            if actual.kind == TokenType::STATIC {
                self.assert(TokenType::STATIC);
                self.parse_type();
                self.parse_var_name();
                self.assert(TokenType::SEMICOLON);

            }
            else if actual.kind == TokenType::FIELD {
                self.assert(TokenType::FIELD);
                self.parse_type();
                self.parse_var_name();
                self.assert(TokenType::SEMICOLON);

            }
        
        } else {
            
        }
        self.close_tag("classVarDec");
        Ok(())   
    }
    fn parse_type(&mut self) -> Result<(), ParserError>{
        let types = [TokenType::INT, TokenType::CHAR, TokenType::BOOLEAN];
        let actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        if types.contains(&actual.kind){
            if actual.kind == TokenType::INT{
                self.assert(TokenType::INT);
            } else if actual.kind == TokenType::CHAR{
                self.assert(TokenType::CHAR);
            } else if actual.kind == TokenType::BOOLEAN{
                self.assert(TokenType::BOOLEAN);
            }  
        } else {
            self.parse_class_name();
        }
        
        Ok(())
    }
    fn parse_class_name(&mut self) -> Result<(), ParserError>{
        self.assert(TokenType::IDENT);
        
        Ok(())
    }
    fn parse_var_name(&mut self) -> Result<(), ParserError>{
        self.assert(TokenType::IDENT);
        let mut actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        while actual.kind == TokenType::COMMA {
            self.assert(TokenType::COMMA);
            self.assert(TokenType::IDENT);
            actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        }
        
        Ok(())
    }
    pub fn parse_expression(&mut self) -> Result<(), ParserError>{
        let mut actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        //println!("{}",actual.kind);
         self.open_tag("expression");
        
            
        self.parse_term()?;
            
        let operators = [TokenType::PLUS,TokenType::MINUS,TokenType::ASTERISK,
                TokenType::SLASH, TokenType::AND,TokenType::OR,TokenType::LT, TokenType::GT, TokenType::EQ,];
        actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        while operators.contains(&actual.kind){
            //println!("{}",actual.kind);
            self.assert(actual.kind);
            self.parse_term()?;
            actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        }
        self.close_tag("expression");
        
        Ok(())
    }
    pub fn parse_term(&mut self) -> Result<(), ParserError>{

        self.open_tag("term");
        let mut actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        let keyword_constant = [TokenType::THIS,TokenType::NULL,TokenType::TRUE,TokenType::FALSE];
        let unary_operator = [TokenType::MINUS, TokenType::NOT];
        if actual.kind == TokenType::NUMBER {
            self.assert(TokenType::NUMBER);
        } else if actual.kind == TokenType::STRING {
            self.assert(TokenType::STRING);
        } else if keyword_constant.contains(&actual.kind){
            self.parse_keywordConstant()?;
        } else if actual.kind == TokenType::LPAREN {
            self.assert(TokenType::LPAREN);
            self.parse_expression()?;
            self.assert(TokenType::RPAREN);
            
        } else if unary_operator.contains(&actual.kind){
            self.parse_unary();
            self.parse_term();
        }
        

        self.close_tag("term");
        Ok(())
    }

    

    fn parse_keywordConstant(&mut self) -> Result<(), ParserError>{
        let actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        if actual.kind == TokenType::THIS{
            self.assert(TokenType::THIS);
        } else if actual.kind == TokenType::NULL{
            self.assert(TokenType::NULL);
        } else if actual.kind == TokenType::TRUE{
            self.assert(TokenType::TRUE);
        } else if actual.kind == TokenType::FALSE{
            self.assert(TokenType::FALSE);
        }
        
        Ok(())
    }
    fn parse_unary(&mut self)-> Result<(), ParserError>{
        let actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        if actual.kind == TokenType::NOT{
            self.assert(TokenType::NOT);
        } else if actual.kind == TokenType::MINUS{
            self.assert(TokenType::MINUS);
        }
        Ok(())
        
    }
    pub fn parse_expression_list(&mut self)->Result<(), ParserError>{
        self.open_tag("expressionList");
        let mut actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        if actual.kind == TokenType::RPAREN{
            self.close_tag("expressionList");
            return  Ok(());
        }
        self.parse_expression();
        actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;
        while actual.kind == TokenType::COMMA {
            self.assert(TokenType::COMMA);
            self.parse_expression();
            actual = self.peek(0).ok_or(ParserError::UnexpectedEOF)?;     
        }
        self.close_tag("expressionList");
        Ok(())
    }
}
