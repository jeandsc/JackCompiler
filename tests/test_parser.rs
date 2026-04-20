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
#[test]
fn test_parse_class_with_robust_class_var_decs() {
    let tokens = vec![
        Token { kind: TokenType::CLASS, lexeme: "class".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Main".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        
        // 1) static int i, j; (múltiplas variáveis)
        Token { kind: TokenType::STATIC, lexeme: "static".to_string(), line: 2 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 2 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        
        // 2) field boolean flag;
        Token { kind: TokenType::FIELD, lexeme: "field".to_string(), line: 3 },
        Token { kind: TokenType::BOOLEAN, lexeme: "boolean".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "flag".to_string(), line: 3 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 3 },
        
        // 3) static char ch;
        Token { kind: TokenType::STATIC, lexeme: "static".to_string(), line: 4 },
        Token { kind: TokenType::CHAR, lexeme: "char".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "ch".to_string(), line: 4 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 4 },
        
        // 4) field String name;
        Token { kind: TokenType::FIELD, lexeme: "field".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "String".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "name".to_string(), line: 5 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 5 },
        
        // 5) static SquareGame game1, game2; (múltiplas variáveis de tipo classe)
        Token { kind: TokenType::STATIC, lexeme: "static".to_string(), line: 6 },
        Token { kind: TokenType::IDENT, lexeme: "SquareGame".to_string(), line: 6 },
        Token { kind: TokenType::IDENT, lexeme: "game1".to_string(), line: 6 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 6 },
        Token { kind: TokenType::IDENT, lexeme: "game2".to_string(), line: 6 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 6 },
        
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 7 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 8 },
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
    <identifier> i </identifier>
    <symbol> , </symbol>
    <identifier> j </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <classVarDec>
    <keyword> field </keyword>
    <keyword> boolean </keyword>
    <identifier> flag </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <classVarDec>
    <keyword> static </keyword>
    <keyword> char </keyword>
    <identifier> ch </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <classVarDec>
    <keyword> field </keyword>
    <identifier> String </identifier>
    <identifier> name </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <classVarDec>
    <keyword> static </keyword>
    <identifier> SquareGame </identifier>
    <identifier> game1 </identifier>
    <symbol> , </symbol>
    <identifier> game2 </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <symbol> } </symbol>
</class>"#;
    
    assert_eq!(parser.get_xml(), expected);
}
}