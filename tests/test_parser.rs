#[cfg(test)]
mod tests {
    use JackCompiler::scanner::{Scanner};
    use JackCompiler::parser::{Parser}; 
    use JackCompiler::token::{Token, TokenType};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parse_empty_class() {
        let tokens = vec![
            Token { kind: TokenType::CLASS, lexeme: "class".to_string(), line: 1 },
            Token { kind: TokenType::IDENT, lexeme: "Main".to_string(), line: 1 },
            Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
            Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 1 },
            Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
        ];
        let mut parser = Parser::new(tokens);
        parser.parse_code();
       
        let expected = r#"<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <symbol> } </symbol>
</class>"#;
        assert_eq!(parser.get_xml(), expected);
    }
    #[test]
fn test_parse_class_with_static_int() {
    let tokens = vec![
        Token { kind: TokenType::CLASS, lexeme: "class".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Main".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        Token { kind: TokenType::STATIC, lexeme: "static".to_string(), line: 2 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 3 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 4 },
    ];

    let mut parser = Parser::new(tokens);
    parser.parse_code();

    let expected = r#"<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <classVarDec>
    <keyword> static </keyword>
    <keyword> int </keyword>
    <identifier> x </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <symbol> } </symbol>
</class>"#;

    assert_eq!(parser.get_xml(), expected);
}
}