#[cfg(test)]
mod tests {
    use JackCompiler::scanner::{Scanner}; 
    use JackCompiler::token::{Token, TokenType};


    #[test]
    fn teste_numero_basico() {
        let code = "322".to_string();
        let mut scanner = Scanner::new(code);
        let tokens:&Vec<Token> = scanner.tokenize();

        assert_eq!(tokens[0].kind, TokenType::NUMBER);
        assert_eq!(tokens[0].lexeme, "322");

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

        assert_eq!(tokens[1].kind, TokenType::NUMBER);
        assert_eq!(tokens[1].lexeme, "34");

        assert_eq!(tokens[2].kind, TokenType::NUMBER);
        assert_eq!(tokens[2].lexeme, "56");

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

    }
    #[test]
    fn test_numeros_e_strings() {
    let code = r#"123"hello""world"456"#.to_string();
    println!("{}", code);
    let mut scanner = Scanner::new(code);
    let tokens = scanner.tokenize();

    assert_eq!(tokens[0].kind, TokenType::NUMBER);
    assert_eq!(tokens[0].lexeme, "123".to_string());

    assert_eq!(tokens[1].kind, TokenType::STRING);
    assert_eq!(tokens[1].lexeme, "hello".to_string());

    assert_eq!(tokens[2].kind, TokenType::STRING);
    assert_eq!(tokens[2].lexeme, "world".to_string());

    assert_eq!(tokens[3].kind, TokenType::NUMBER);
    assert_eq!(tokens[3].lexeme, "456".to_string());

    println!("✅ Teste de número + strings passou!");
    }
}
