#[cfg(test)]
mod tests {
    use JackCompiler::scanner::{Scanner}; 
    use JackCompiler::token::{Token, TokenType};


    #[test]
    fn teste_numero_basico() {
        let code = "322";
        let mut scanner = Scanner::new(code);
        let tokens:&Vec<Token> = scanner.tokenize();

        assert_eq!(tokens[0].kind, TokenType::NUMBER);

    }
}