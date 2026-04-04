pub mod xml_generator;
pub mod token;
pub mod scanner;


use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;



use crate::scanner::{ Scanner};
use crate::xml_generator::generate_xml;

fn salvar_xml(pasta: &str, nome_arquivo: &str, conteudo: &str) -> std::io::Result<()> {
    
    fs::create_dir_all(pasta)?;
    
    
    let caminho = Path::new(pasta).join(nome_arquivo);
    
    
    let mut file = File::create(caminho)?;
    file.write_all(conteudo.as_bytes())?;
    
    Ok(())
}


fn main() {
    let pasta = "tests/nand2tetris_files/Square";
    let mut arquivos_jack:Vec<String> = Vec::new();
    let mut arquivos_referencia:Vec<String> = Vec::new();

    for entry in fs::read_dir(pasta).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name(); 
            let file_str = file_name.to_string_lossy().to_string(); 
            
            if file_str.ends_with(".jack") {
                arquivos_jack.push(file_str.clone()); 
                arquivos_referencia.push(file_str.replace(".jack", "T.xml"));
            }
    }
        
    for i in 0..arquivos_jack.len(){
            let path_jack = format!("{}/{}",pasta, arquivos_jack[i]);
            let path_referencia = format!("{}/{}",pasta, arquivos_referencia[i]);
            let output = "output/Square";
            
            let code =fs::read_to_string(path_jack).expect("Falha ao ler arquivo");
            let mut xml_referencia = fs::read_to_string(path_referencia).expect("Falha ao ler arquivo");
            let scanner = Scanner::new(code);
            let mut xml_gerado = generate_xml(scanner);

            if let Err(e) = salvar_xml(output, &format!("{}", arquivos_jack[i]).replace(".jack", "T.xml"), &xml_gerado) {
                println!("Erro ao salvar XML: {}", e);
            }
            
            xml_gerado = xml_gerado.replace("\r\n", "\n");
            xml_referencia = xml_referencia.replace("\r\n", "\n");
           
            assert_eq!(xml_gerado, xml_referencia);

            
    }
}
