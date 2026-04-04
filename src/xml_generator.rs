use crate::token::{ TokenType};
use crate::scanner::{ Scanner};
use std::fs;

pub fn generate_xml(mut scanner:Scanner) ->String{
    let tokens = scanner.tokenize();
    let mut xml = String::new();
    xml+= "<tokens>\r\n";
    for token in tokens.iter().filter(|t| t.kind != TokenType::EOF) {
        xml+= format!("{}\r\n", token.to_xml()).as_str();
    }
    xml+="</tokens>\r\n";
    return xml;
}


