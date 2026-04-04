pub mod xml_generator;
pub mod token;
pub mod scanner;


use std::fs;


use crate::scanner::{ Scanner};
use crate::xml_generator::generate_xml;


fn main() {
    let code =fs::read_to_string("tests/nand2tetris_files/Square/Main.jack")
    .expect("Falha ao ler arquivo");
    
}
