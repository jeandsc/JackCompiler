#[cfg(test)]
mod tests {
    use JackCompiler::scanner::{Scanner}; 
    use JackCompiler::token::{Token, TokenType};
    use std::fs;
    use std::path::Path;
    use JackCompiler::xml_generator::generate_xml;


    #[test]
    fn teste_numero_basico() {
        let code = "322".to_string();
        let mut scanner = Scanner::new(code);
        let tokens:&Vec<Token> = scanner.tokenize();

        assert_eq!(tokens[0].kind, TokenType::NUMBER);
        assert_eq!(tokens[0].lexeme, "322");
        assert_eq!(tokens[0].to_xml(), "<integerConstant> 322 </integerConstant>");

        println!("Teste Passou!")

    }
    #[test]
    fn teste_multiplos_numeros() {
        let code = "12 34 56".to_string();
        let mut scanner = Scanner::new(code);

        let tokens = scanner.tokenize(); // preenche scanner.tokens

        // Verifica que temos 3 tokens
        assert_eq!(tokens.len(), 4);

        // Verifica cada token individualmente
        assert_eq!(tokens[0].kind, TokenType::NUMBER);
        assert_eq!(tokens[0].lexeme, "12");
        assert_eq!(tokens[0].to_xml(), "<integerConstant> 12 </integerConstant>");

        assert_eq!(tokens[1].kind, TokenType::NUMBER);
        assert_eq!(tokens[1].lexeme, "34");
        assert_eq!(tokens[1].to_xml(), "<integerConstant> 34 </integerConstant>");

        assert_eq!(tokens[2].kind, TokenType::NUMBER);
        assert_eq!(tokens[2].lexeme, "56");
        assert_eq!(tokens[2].to_xml(), "<integerConstant> 56 </integerConstant>");

        assert_eq!(tokens[3].kind, TokenType::EOF);
        assert_eq!(tokens[3].lexeme, "");

        println!("✅ Teste de múltiplos números passou!");
    }
    #[test]
    fn test_string_basica(){
        let code = r#""hello""#.to_string();
        println!("{}", code);
        let mut scanner = Scanner::new(code);
        let tokens = scanner.tokenize();
        
        assert_eq!(tokens[0].kind, TokenType::STRING);
        assert_eq!(tokens[0].lexeme, r#"hello"#.to_string());
        assert_eq!(tokens[0].to_xml(), "<stringConstant> hello </stringConstant>");
    }
    #[test]
    fn test_numeros_e_strings() {
    let code = r#"123" hello""world"456"#.to_string();
    println!("{}", code);
    let mut scanner = Scanner::new(code);
    let tokens = scanner.tokenize();

    assert_eq!(tokens[0].kind, TokenType::NUMBER);
    assert_eq!(tokens[0].lexeme, "123".to_string());

    assert_eq!(tokens[1].kind, TokenType::STRING);
    assert_eq!(tokens[1].lexeme, " hello".to_string());

    assert_eq!(tokens[2].kind, TokenType::STRING);
    assert_eq!(tokens[2].lexeme, "world".to_string());

    assert_eq!(tokens[3].kind, TokenType::NUMBER);
    assert_eq!(tokens[3].lexeme, "456".to_string());

    println!("✅ Teste de número + strings passou!");
    }

    #[test]
    fn teste_identificadores_e_keywords(){
        let mut code = "minhaVar123".to_string();
        let mut scanner = Scanner::new(code);
        let mut tokens = scanner.tokenize();

        assert_eq!(tokens[0].kind, TokenType::IDENT);
        assert_eq!(tokens[0].lexeme, "minhaVar123".to_string());
        assert_eq!(tokens[0].to_xml(), "<identifier> minhaVar123 </identifier>");

        code = "function".to_string();
        scanner = Scanner::new(code);
        tokens = scanner.tokenize();

        assert_eq!(tokens[0].kind, TokenType::FUNCTION);
        assert_eq!(tokens[0].lexeme, "function".to_string());
        assert_eq!(tokens[0].to_xml(), "<keyword> function </keyword>");

        

    }
    #[test]
    fn test_symbolos_xml(){
        let code = "x+y;".to_string();
        let mut scanner = Scanner::new(code);
        let tokens = scanner.tokenize();

        let lista_xmls = vec![
        "<identifier> x </identifier>",
        "<symbol> + </symbol>",
        "<identifier> y </identifier>",
        "<symbol> ; </symbol>"];
        
        for i in 0..lista_xmls.len(){
            assert_eq!(tokens[i].to_xml(), lista_xmls[i])
        }
    }
    #[test]
    fn teste_ignorar_comentarios(){
        let code = "let x = 5; // isto some".to_string();
        let mut scanner = Scanner::new(code);
        let tokens = scanner.tokenize();

        assert_eq!(tokens.len(), 6);

    }
    #[test]
    fn teste_ignorar_comentario_multilinha(){
        let code = "let x = 5; /* isto deve ser ignorado */ let y = 10;".to_string();
        let mut scanner = Scanner::new(code);
        let tokens = scanner.tokenize();

    
        assert_eq!(tokens.len(), 11);
    }
    #[test]
    fn teste_codigo_jack_completo() {
    let code = r#"
    class Main {
        function void main() {
            let x = 5;
            return;
        }
    }
    "#.to_string();

    let mut scanner = Scanner::new(code);
    let tokens = scanner.tokenize();

    
    let tipos: Vec<TokenType> = tokens.iter().map(|t| t.kind).collect();
    let lexemes: Vec<String> = tokens.iter().map(|t| t.lexeme.clone()).collect();

   
    assert!(tipos.contains(&TokenType::CLASS));
    assert!(tipos.contains(&TokenType::FUNCTION));
    assert!(tipos.contains(&TokenType::IDENT));
    assert!(tipos.contains(&TokenType::NUMBER));
    assert!(tipos.contains(&TokenType::EQ));

    
    assert!(lexemes.contains(&"Main".to_string()));
    assert!(lexemes.contains(&"x".to_string()));
    assert!(lexemes.contains(&"5".to_string()));
    }
    #[test]
    fn test_validacao_nand2tetris_square_main(){
        let jack_path = "tests/nand2tetris_files/Square/Main.jack";
        let xml_referencia_path = "tests/nand2tetris_files/Square/MainT.xml";
        assert!(
            Path::new(jack_path).exists(),
            "Arquivo Jack não encontrado: {}",
            jack_path
        );

        assert!(
            Path::new(xml_referencia_path).exists(),
            "Arquivo XML de referência não encontrado: {}",
            xml_referencia_path
        );
        let code =fs::read_to_string(jack_path).expect("Falha ao ler arquivo");
        let mut xml_referencia = fs::read_to_string(xml_referencia_path).expect("Falha ao ler arquivo");
        let mut scanner = Scanner::new(code);
        let mut xml_gerado = generate_xml(scanner);

        xml_gerado = xml_gerado.replace("\r\n", "\n");
        xml_referencia = xml_referencia.replace("\r\n", "\n");

        assert_eq!(xml_gerado, xml_referencia);
        println!("Passou!")


    }
    #[test]
    fn test_todos_os_codigos_nand2tetris(){
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
            
            let code =fs::read_to_string(path_jack).expect("Falha ao ler arquivo");
            let mut xml_referencia = fs::read_to_string(path_referencia).expect("Falha ao ler arquivo");
            let scanner = Scanner::new(code);
            let mut xml_gerado = generate_xml(scanner);

            xml_gerado = xml_gerado.replace("\r\n", "\n");
            xml_referencia = xml_referencia.replace("\r\n", "\n");
           
            assert_eq!(xml_gerado, xml_referencia);
            
        }
    }
}
