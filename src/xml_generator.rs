use crate::token::{Token, TokenType};
use crate::scanner::{self, Scanner};
use std::fs;

pub fn generate_xml(mut scanner:Scanner) {
    let tokens = scanner.tokenize();
    let mut xml = String::new();
    xml+= "<token>\n";
    for token in tokens.iter().filter(|t| t.kind != TokenType::EOF) {
        xml+= format!("{}\n", token.to_xml()).as_str();
    }
    xml+="</token>";
    fs::write("tests/output/Square/MainT.xml", xml)
        .expect("Erro ao escrever o arquivo");
}


