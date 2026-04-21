#[cfg(test)]
mod tests {
    use JackCompiler::scanner::{Scanner};
    use JackCompiler::parser::{self, Parser}; 
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

#[test]
fn test_parse_term_integer_constant() {
    let tokens = vec![
        Token { kind: TokenType::NUMBER, lexeme: "123".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <integerConstant> 123 </integerConstant>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_string_constant() {
    let tokens = vec![
        Token { kind: TokenType::STRING, lexeme: "hello world".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <stringConstant> hello world </stringConstant>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_keyword_constant_true() {
    let tokens = vec![
        Token { kind: TokenType::TRUE, lexeme: "true".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <keyword> true </keyword>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}
#[test]
fn test_parse_expression_no_parentheses() {
    let tokens = vec![
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap();

    let expected = r#"<expression>
  <term>
    <integerConstant> 1 </integerConstant>
  </term>
  <symbol> + </symbol>
  <term>
    <integerConstant> 2 </integerConstant>
  </term>
  <symbol> * </symbol>
  <term>
    <integerConstant> 3 </integerConstant>
  </term>
</expression>"#;

    assert_eq!(parser.get_xml(), expected);
}
#[test]
fn test_parse_expression_mixed_terms_no_parentheses() {
    let tokens = vec![
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::STRING, lexeme: "hello".to_string(), line: 1 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 1 },
        Token { kind: TokenType::TRUE, lexeme: "true".to_string(), line: 1 },
        Token { kind: TokenType::MINUS, lexeme: "-".to_string(), line: 1 },
        Token { kind: TokenType::FALSE, lexeme: "false".to_string(), line: 1 },
        Token { kind: TokenType::SLASH, lexeme: "/".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::AND, lexeme: "&".to_string(), line: 1 },
        Token { kind: TokenType::NULL, lexeme: "null".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap();

    let expected = r#"<expression>
  <term>
    <integerConstant> 1 </integerConstant>
  </term>
  <symbol> + </symbol>
  <term>
    <stringConstant> hello </stringConstant>
  </term>
  <symbol> * </symbol>
  <term>
    <keyword> true </keyword>
  </term>
  <symbol> - </symbol>
  <term>
    <keyword> false </keyword>
  </term>
  <symbol> / </symbol>
  <term>
    <integerConstant> 2 </integerConstant>
  </term>
  <symbol> &amp; </symbol>
  <term>
    <keyword> null </keyword>
  </term>
</expression>"#;

    assert_eq!(parser.get_xml(), expected);
}
#[test]
fn test_parse_expression_with_parentheses() {
    let tokens = vec![
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::TRUE, lexeme: "true".to_string(), line: 1 },
        Token { kind: TokenType::AND, lexeme: "&".to_string(), line: 1 },
        Token { kind: TokenType::FALSE, lexeme: "false".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::OR, lexeme: "|".to_string(), line: 1 },
        Token { kind: TokenType::STRING, lexeme: "hello".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap();

    let expected = r#"<expression>
  <term>
    <symbol> ( </symbol>
    <expression>
      <term>
        <keyword> true </keyword>
      </term>
      <symbol> &amp; </symbol>
      <term>
        <keyword> false </keyword>
      </term>
    </expression>
    <symbol> ) </symbol>
  </term>
  <symbol> | </symbol>
  <term>
    <stringConstant> hello </stringConstant>
  </term>
</expression>"#;

    assert_eq!(parser.get_xml(), expected);
}
#[test]
fn test_parse_expression_unary_operators() {
    let tokens = vec![
        Token { kind: TokenType::MINUS, lexeme: "-".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "5".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NOT, lexeme: "~".to_string(), line: 1 },
        Token { kind: TokenType::TRUE, lexeme: "true".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap();

    let expected = r#"<expression>
  <term>
    <symbol> - </symbol>
    <term>
      <integerConstant> 5 </integerConstant>
    </term>
  </term>
  <symbol> + </symbol>
  <term>
    <symbol> ~ </symbol>
    <term>
      <keyword> true </keyword>
    </term>
  </term>
</expression>"#;

    assert_eq!(parser.get_xml(), expected);
}
#[test]
fn test_parse_expression_list_empty() {
    let tokens = vec![
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression_list().unwrap();
    let expected = r#"<expressionList>
</expressionList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_expression_list_one() {
    let tokens = vec![
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression_list().unwrap();
    let expected = r#"<expressionList>
  <expression>
    <term>
      <integerConstant> 1 </integerConstant>
    </term>
  </expression>
</expressionList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_expression_list_two() {
    let tokens = vec![
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression_list().unwrap();
    let expected = r#"<expressionList>
  <expression>
    <term>
      <integerConstant> 1 </integerConstant>
    </term>
  </expression>
  <symbol> , </symbol>
  <expression>
    <term>
      <integerConstant> 2 </integerConstant>
    </term>
  </expression>
</expressionList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_expression_list_complex_no_varname_no_outer_parens() {
    let tokens = vec![
        // Primeira expressão: (1+2)*3  (note que esta expressão tem seus próprios parênteses internos)
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        // Segunda expressão: -5 + ~true
        Token { kind: TokenType::MINUS, lexeme: "-".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "5".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NOT, lexeme: "~".to_string(), line: 1 },
        Token { kind: TokenType::TRUE, lexeme: "true".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        // Terceira expressão: "hello" & false
        Token { kind: TokenType::STRING, lexeme: "hello".to_string(), line: 1 },
        Token { kind: TokenType::AND, lexeme: "&".to_string(), line: 1 },
        Token { kind: TokenType::FALSE, lexeme: "false".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression_list().unwrap();

    let expected = r#"<expressionList>
  <expression>
    <term>
      <symbol> ( </symbol>
      <expression>
        <term>
          <integerConstant> 1 </integerConstant>
        </term>
        <symbol> + </symbol>
        <term>
          <integerConstant> 2 </integerConstant>
        </term>
      </expression>
      <symbol> ) </symbol>
    </term>
    <symbol> * </symbol>
    <term>
      <integerConstant> 3 </integerConstant>
    </term>
  </expression>
  <symbol> , </symbol>
  <expression>
    <term>
      <symbol> - </symbol>
      <term>
        <integerConstant> 5 </integerConstant>
      </term>
    </term>
    <symbol> + </symbol>
    <term>
      <symbol> ~ </symbol>
      <term>
        <keyword> true </keyword>
      </term>
    </term>
  </expression>
  <symbol> , </symbol>
  <expression>
    <term>
      <stringConstant> hello </stringConstant>
    </term>
    <symbol> &amp; </symbol>
    <term>
      <keyword> false </keyword>
    </term>
  </expression>
</expressionList>"#;

    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_array_indexing_simple() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 1 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <identifier> a </identifier>
  <symbol> [ </symbol>
  <expression>
    <term>
      <integerConstant> 1 </integerConstant>
    </term>
  </expression>
  <symbol> ] </symbol>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_array_indexing_expression() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "b".to_string(), line: 1 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <identifier> b </identifier>
  <symbol> [ </symbol>
  <expression>
    <term>
      <integerConstant> 2 </integerConstant>
    </term>
    <symbol> + </symbol>
    <term>
      <integerConstant> 3 </integerConstant>
    </term>
  </expression>
  <symbol> ] </symbol>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_expression_complex_with_arrays() {
    let tokens = vec![
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 1 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "b".to_string(), line: 1 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "0".to_string(), line: 1 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap();
    let expected = r#"<expression>
  <term>
    <symbol> ( </symbol>
    <expression>
      <term>
        <identifier> a </identifier>
        <symbol> [ </symbol>
        <expression>
          <term>
            <integerConstant> 2 </integerConstant>
          </term>
        </expression>
        <symbol> ] </symbol>
      </term>
      <symbol> + </symbol>
      <term>
        <integerConstant> 3 </integerConstant>
      </term>
    </expression>
    <symbol> ) </symbol>
  </term>
  <symbol> * </symbol>
  <term>
    <identifier> b </identifier>
    <symbol> [ </symbol>
    <expression>
      <term>
        <integerConstant> 0 </integerConstant>
      </term>
    </expression>
    <symbol> ] </symbol>
  </term>
</expression>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_subroutine_call_no_args() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <identifier> foo </identifier>
  <symbol> ( </symbol>
  <expressionList>
  </expressionList>
  <symbol> ) </symbol>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_subroutine_call_with_args() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::STRING, lexeme: "hello".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <identifier> foo </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <integerConstant> 1 </integerConstant>
      </term>
    </expression>
    <symbol> , </symbol>
    <expression>
      <term>
        <identifier> x </identifier>
      </term>
      <symbol> + </symbol>
      <term>
        <integerConstant> 2 </integerConstant>
      </term>
    </expression>
    <symbol> , </symbol>
    <expression>
      <term>
        <stringConstant> hello </stringConstant>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_method_call_no_args() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "obj".to_string(), line: 1 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <identifier> obj </identifier>
  <symbol> . </symbol>
  <identifier> method </identifier>
  <symbol> ( </symbol>
  <expressionList>
  </expressionList>
  <symbol> ) </symbol>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_term_method_call_with_args() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "obj".to_string(), line: 1 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_term().unwrap();
    let expected = r#"<term>
  <identifier> obj </identifier>
  <symbol> . </symbol>
  <identifier> method </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <integerConstant> 3 </integerConstant>
      </term>
    </expression>
    <symbol> , </symbol>
    <expression>
      <term>
        <identifier> y </identifier>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
</term>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_expression_complex_with_method_call() {
    let tokens = vec![
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "obj".to_string(), line: 1 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "4".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_expression().unwrap();
    let expected = r#"<expression>
  <term>
    <symbol> ( </symbol>
    <expression>
      <term>
        <identifier> obj </identifier>
        <symbol> . </symbol>
        <identifier> method </identifier>
        <symbol> ( </symbol>
        <expressionList>
          <expression>
            <term>
              <integerConstant> 2 </integerConstant>
            </term>
          </expression>
        </expressionList>
        <symbol> ) </symbol>
      </term>
      <symbol> + </symbol>
      <term>
        <integerConstant> 3 </integerConstant>
      </term>
    </expression>
    <symbol> ) </symbol>
  </term>
  <symbol> * </symbol>
  <term>
    <integerConstant> 4 </integerConstant>
  </term>
</expression>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_return_statement_no_expression() {
    let tokens = vec![
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_return_statement().unwrap();
    let expected = r#"<returnStatement>
  <keyword> return </keyword>
  <symbol> ; </symbol>
</returnStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_return_statement_with_expression() {
    let tokens = vec![
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "42".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_return_statement().unwrap();
    let expected = r#"<returnStatement>
  <keyword> return </keyword>
  <expression>
    <term>
      <integerConstant> 42 </integerConstant>
    </term>
  </expression>
  <symbol> ; </symbol>
</returnStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_do_statement_simple_call() {
    let tokens = vec![
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_do_statement().unwrap();
    let expected = r#"<doStatement>
  <keyword> do </keyword>
  <identifier> foo </identifier>
  <symbol> ( </symbol>
  <expressionList>
  </expressionList>
  <symbol> ) </symbol>
  <symbol> ; </symbol>
</doStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_do_statement_method_call() {
    let tokens = vec![
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "obj".to_string(), line: 1 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_do_statement().unwrap();
    let expected = r#"<doStatement>
  <keyword> do </keyword>
  <identifier> obj </identifier>
  <symbol> . </symbol>
  <identifier> method </identifier>
  <symbol> ( </symbol>
  <expressionList>
  </expressionList>
  <symbol> ) </symbol>
  <symbol> ; </symbol>
</doStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_do_statement_static_call_with_args() {
    let tokens = vec![
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Math".to_string(), line: 1 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "sqrt".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "4".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_do_statement().unwrap();
    let expected = r#"<doStatement>
  <keyword> do </keyword>
  <identifier> Math </identifier>
  <symbol> . </symbol>
  <identifier> sqrt </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <integerConstant> 4 </integerConstant>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
  <symbol> ; </symbol>
</doStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_do_statement_with_multiple_args() {
    let tokens = vec![
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_do_statement().unwrap();
    let expected = r#"<doStatement>
  <keyword> do </keyword>
  <identifier> foo </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <integerConstant> 1 </integerConstant>
      </term>
    </expression>
    <symbol> , </symbol>
    <expression>
      <term>
        <identifier> x </identifier>
      </term>
      <symbol> + </symbol>
      <term>
        <integerConstant> 2 </integerConstant>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
  <symbol> ; </symbol>
</doStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_let_statement_simple() {
    let tokens = vec![
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "5".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_let_statement().unwrap();
    let expected = r#"<letStatement>
  <keyword> let </keyword>
  <identifier> x </identifier>
  <symbol> = </symbol>
  <expression>
    <term>
      <integerConstant> 5 </integerConstant>
    </term>
  </expression>
  <symbol> ; </symbol>
</letStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_let_statement_with_array_index() {
    let tokens = vec![
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 1 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 1 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_let_statement().unwrap();
    let expected = r#"<letStatement>
  <keyword> let </keyword>
  <identifier> a </identifier>
  <symbol> [ </symbol>
  <expression>
    <term>
      <integerConstant> 1 </integerConstant>
    </term>
  </expression>
  <symbol> ] </symbol>
  <symbol> = </symbol>
  <expression>
    <term>
      <integerConstant> 2 </integerConstant>
    </term>
  </expression>
  <symbol> ; </symbol>
</letStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_let_statement_complex_expression() {
    let tokens = vec![
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "3".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_let_statement().unwrap();
    let expected = r#"<letStatement>
  <keyword> let </keyword>
  <identifier> x </identifier>
  <symbol> = </symbol>
  <expression>
    <term>
      <identifier> y </identifier>
    </term>
    <symbol> + </symbol>
    <term>
      <integerConstant> 3 </integerConstant>
    </term>
  </expression>
  <symbol> ; </symbol>
</letStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_let_statement_array_with_expression_index() {
    let tokens = vec![
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "arr".to_string(), line: 1 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 1 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 1 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 1 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 1 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_let_statement().unwrap();
    let expected = r#"<letStatement>
  <keyword> let </keyword>
  <identifier> arr </identifier>
  <symbol> [ </symbol>
  <expression>
    <term>
      <identifier> i </identifier>
    </term>
    <symbol> + </symbol>
    <term>
      <integerConstant> 1 </integerConstant>
    </term>
  </expression>
  <symbol> ] </symbol>
  <symbol> = </symbol>
  <expression>
    <term>
      <identifier> j </identifier>
    </term>
    <symbol> * </symbol>
    <term>
      <integerConstant> 2 </integerConstant>
    </term>
  </expression>
  <symbol> ; </symbol>
</letStatement>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_statements_do_let_return() {
    let tokens = vec![
        // doStatement: do foo();
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        // letStatement: let x = 5;
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "5".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        // returnStatement: return;
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_statements().unwrap();
    let expected = r#"<statements>
  <doStatement>
    <keyword> do </keyword>
    <identifier> foo </identifier>
    <symbol> ( </symbol>
    <expressionList>
    </expressionList>
    <symbol> ) </symbol>
    <symbol> ; </symbol>
  </doStatement>
  <letStatement>
    <keyword> let </keyword>
    <identifier> x </identifier>
    <symbol> = </symbol>
    <expression>
      <term>
        <integerConstant> 5 </integerConstant>
      </term>
    </expression>
    <symbol> ; </symbol>
  </letStatement>
  <returnStatement>
    <keyword> return </keyword>
    <symbol> ; </symbol>
  </returnStatement>
</statements>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_statements_let_do_return() {
    let tokens = vec![
        // letStatement: let y = 10;
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 1 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "10".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        // doStatement: do obj.method();
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "obj".to_string(), line: 1 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        // returnStatement: return 0;
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "0".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_statements().unwrap();
    let expected = r#"<statements>
  <letStatement>
    <keyword> let </keyword>
    <identifier> y </identifier>
    <symbol> = </symbol>
    <expression>
      <term>
        <integerConstant> 10 </integerConstant>
      </term>
    </expression>
    <symbol> ; </symbol>
  </letStatement>
  <doStatement>
    <keyword> do </keyword>
    <identifier> obj </identifier>
    <symbol> . </symbol>
    <identifier> method </identifier>
    <symbol> ( </symbol>
    <expressionList>
    </expressionList>
    <symbol> ) </symbol>
    <symbol> ; </symbol>
  </doStatement>
  <returnStatement>
    <keyword> return </keyword>
    <expression>
      <term>
        <integerConstant> 0 </integerConstant>
      </term>
    </expression>
    <symbol> ; </symbol>
  </returnStatement>
</statements>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_statements_empty() {
    let tokens = vec![
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_statements().unwrap();
    let expected = r#"<statements>
</statements>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_statements_nested_if_while() {
    let tokens = vec![
        // if (x > 0) {
        Token { kind: TokenType::IF, lexeme: "if".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::GT, lexeme: ">".to_string(), line: 1 },
        Token { kind: TokenType::NUMBER, lexeme: "0".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        //   let y = 1;
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 2 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 2 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        //   while (y < 10) {
        Token { kind: TokenType::WHILE, lexeme: "while".to_string(), line: 3 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 3 },
        Token { kind: TokenType::LT, lexeme: "<".to_string(), line: 3 },
        Token { kind: TokenType::NUMBER, lexeme: "10".to_string(), line: 3 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 3 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 3 },
        //       let y = y + 1;
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 4 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "y".to_string(), line: 4 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 4 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 4 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 4 },
        //     }
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 5 },
        //   }
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 6 },
        // } else {
        Token { kind: TokenType::ELSE, lexeme: "else".to_string(), line: 7 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 7 },
        //   do foo();
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 8 },
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 8 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 8 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 8 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 8 },
        // }
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 9 },
        // return;
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 10 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 10 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 11 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_statements().unwrap();

    let expected = r#"<statements>
  <ifStatement>
    <keyword> if </keyword>
    <symbol> ( </symbol>
    <expression>
      <term>
        <identifier> x </identifier>
      </term>
      <symbol> &gt; </symbol>
      <term>
        <integerConstant> 0 </integerConstant>
      </term>
    </expression>
    <symbol> ) </symbol>
    <symbol> { </symbol>
    <statements>
      <letStatement>
        <keyword> let </keyword>
        <identifier> y </identifier>
        <symbol> = </symbol>
        <expression>
          <term>
            <integerConstant> 1 </integerConstant>
          </term>
        </expression>
        <symbol> ; </symbol>
      </letStatement>
      <whileStatement>
        <keyword> while </keyword>
        <symbol> ( </symbol>
        <expression>
          <term>
            <identifier> y </identifier>
          </term>
          <symbol> &lt; </symbol>
          <term>
            <integerConstant> 10 </integerConstant>
          </term>
        </expression>
        <symbol> ) </symbol>
        <symbol> { </symbol>
        <statements>
          <letStatement>
            <keyword> let </keyword>
            <identifier> y </identifier>
            <symbol> = </symbol>
            <expression>
              <term>
                <identifier> y </identifier>
              </term>
              <symbol> + </symbol>
              <term>
                <integerConstant> 1 </integerConstant>
              </term>
            </expression>
            <symbol> ; </symbol>
          </letStatement>
        </statements>
        <symbol> } </symbol>
      </whileStatement>
    </statements>
    <symbol> } </symbol>
    <keyword> else </keyword>
    <symbol> { </symbol>
    <statements>
      <doStatement>
        <keyword> do </keyword>
        <identifier> foo </identifier>
        <symbol> ( </symbol>
        <expressionList>
        </expressionList>
        <symbol> ) </symbol>
        <symbol> ; </symbol>
      </doStatement>
    </statements>
    <symbol> } </symbol>
  </ifStatement>
  <returnStatement>
    <keyword> return </keyword>
    <symbol> ; </symbol>
  </returnStatement>
</statements>"#;

    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_var_dec_single() {
    let tokens = vec![
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_var_dec().unwrap();
    let expected = r#"<varDec>
  <keyword> var </keyword>
  <keyword> int </keyword>
  <identifier> x </identifier>
  <symbol> ; </symbol>
</varDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_var_dec_multiple() {
    let tokens = vec![
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "k".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_var_dec().unwrap();
    let expected = r#"<varDec>
  <keyword> var </keyword>
  <keyword> int </keyword>
  <identifier> i </identifier>
  <symbol> , </symbol>
  <identifier> j </identifier>
  <symbol> , </symbol>
  <identifier> k </identifier>
  <symbol> ; </symbol>
</varDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_var_dec_different_types() {
    let tokens = vec![
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "String".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 1 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_var_dec().unwrap();
    let expected = r#"<varDec>
  <keyword> var </keyword>
  <identifier> String </identifier>
  <identifier> s </identifier>
  <symbol> ; </symbol>
</varDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_subroutine_body_evil() {
    let tokens = vec![
        // {
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        // var int i, j;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 2 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 2 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        // var String s;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "String".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 3 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 3 },
        // var Array a, b;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "Array".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 4 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "b".to_string(), line: 4 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 4 },
        // statements: if (i > 0) { let j = 5; } else { do foo(); }
        Token { kind: TokenType::IF, lexeme: "if".to_string(), line: 5 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 5 },
        Token { kind: TokenType::GT, lexeme: ">".to_string(), line: 5 },
        Token { kind: TokenType::NUMBER, lexeme: "0".to_string(), line: 5 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 5 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 5 },
        //   let j = 5;
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 6 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 6 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 6 },
        Token { kind: TokenType::NUMBER, lexeme: "5".to_string(), line: 6 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 6 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 7 },
        Token { kind: TokenType::ELSE, lexeme: "else".to_string(), line: 7 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 7 },
        //   do foo();
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 8 },
        Token { kind: TokenType::IDENT, lexeme: "foo".to_string(), line: 8 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 8 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 8 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 8 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 9 },
        // while (i < 10) { let i = i + 1; }
        Token { kind: TokenType::WHILE, lexeme: "while".to_string(), line: 10 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 10 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 10 },
        Token { kind: TokenType::LT, lexeme: "<".to_string(), line: 10 },
        Token { kind: TokenType::NUMBER, lexeme: "10".to_string(), line: 10 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 10 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 10 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 11 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 11 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 11 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 11 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 11 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 11 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 11 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 12 },
        // return s;
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 13 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 13 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 13 },
        // }
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 14 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 15 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_subroutine_body().unwrap();

    let expected = r#"<subroutineBody>
  <symbol> { </symbol>
  <varDec>
    <keyword> var </keyword>
    <keyword> int </keyword>
    <identifier> i </identifier>
    <symbol> , </symbol>
    <identifier> j </identifier>
    <symbol> ; </symbol>
  </varDec>
  <varDec>
    <keyword> var </keyword>
    <identifier> String </identifier>
    <identifier> s </identifier>
    <symbol> ; </symbol>
  </varDec>
  <varDec>
    <keyword> var </keyword>
    <identifier> Array </identifier>
    <identifier> a </identifier>
    <symbol> , </symbol>
    <identifier> b </identifier>
    <symbol> ; </symbol>
  </varDec>
  <statements>
    <ifStatement>
      <keyword> if </keyword>
      <symbol> ( </symbol>
      <expression>
        <term>
          <identifier> i </identifier>
        </term>
        <symbol> &gt; </symbol>
        <term>
          <integerConstant> 0 </integerConstant>
        </term>
      </expression>
      <symbol> ) </symbol>
      <symbol> { </symbol>
      <statements>
        <letStatement>
          <keyword> let </keyword>
          <identifier> j </identifier>
          <symbol> = </symbol>
          <expression>
            <term>
              <integerConstant> 5 </integerConstant>
            </term>
          </expression>
          <symbol> ; </symbol>
        </letStatement>
      </statements>
      <symbol> } </symbol>
      <keyword> else </keyword>
      <symbol> { </symbol>
      <statements>
        <doStatement>
          <keyword> do </keyword>
          <identifier> foo </identifier>
          <symbol> ( </symbol>
          <expressionList>
          </expressionList>
          <symbol> ) </symbol>
          <symbol> ; </symbol>
        </doStatement>
      </statements>
      <symbol> } </symbol>
    </ifStatement>
    <whileStatement>
      <keyword> while </keyword>
      <symbol> ( </symbol>
      <expression>
        <term>
          <identifier> i </identifier>
        </term>
        <symbol> &lt; </symbol>
        <term>
          <integerConstant> 10 </integerConstant>
        </term>
      </expression>
      <symbol> ) </symbol>
      <symbol> { </symbol>
      <statements>
        <letStatement>
          <keyword> let </keyword>
          <identifier> i </identifier>
          <symbol> = </symbol>
          <expression>
            <term>
              <identifier> i </identifier>
            </term>
            <symbol> + </symbol>
            <term>
              <integerConstant> 1 </integerConstant>
            </term>
          </expression>
          <symbol> ; </symbol>
        </letStatement>
      </statements>
      <symbol> } </symbol>
    </whileStatement>
    <returnStatement>
      <keyword> return </keyword>
      <expression>
        <term>
          <identifier> s </identifier>
        </term>
      </expression>
      <symbol> ; </symbol>
    </returnStatement>
  </statements>
  <symbol> } </symbol>
</subroutineBody>"#;

    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_parameter_list_empty() {
    let tokens = vec![
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 }, // apenas para parar
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_parameter_list().unwrap();
    let expected = r#"<parameterList>
</parameterList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_parameter_list_one() {
    let tokens = vec![
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 }, // stop
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_parameter_list().unwrap();
    let expected = r#"<parameterList>
  <keyword> int </keyword>
  <identifier> x </identifier>
</parameterList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_parameter_list_multiple() {
    let tokens = vec![
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Ax".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Ay".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Asize".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_parameter_list().unwrap();
    let expected = r#"<parameterList>
  <keyword> int </keyword>
  <identifier> Ax </identifier>
  <symbol> , </symbol>
  <keyword> int </keyword>
  <identifier> Ay </identifier>
  <symbol> , </symbol>
  <keyword> int </keyword>
  <identifier> Asize </identifier>
</parameterList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_parameter_list_user_type() {
    let tokens = vec![
        Token { kind: TokenType::IDENT, lexeme: "Square".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "SquareGame".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "game".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 1 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_parameter_list().unwrap();
    let expected = r#"<parameterList>
  <identifier> Square </identifier>
  <identifier> s </identifier>
  <symbol> , </symbol>
  <identifier> SquareGame </identifier>
  <identifier> game </identifier>
</parameterList>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_subroutine_dec_constructor_no_params() {
    let tokens = vec![
        Token { kind: TokenType::CONSTRUCTOR, lexeme: "constructor".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Square".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "new".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 2 },
        Token { kind: TokenType::THIS, lexeme: "this".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 3 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 4 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_subroutine_dec().unwrap();
    let expected = r#"<subroutineDec>
  <keyword> constructor </keyword>
  <identifier> Square </identifier>
  <identifier> new </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <statements>
      <returnStatement>
        <keyword> return </keyword>
        <expression>
          <term>
            <keyword> this </keyword>
          </term>
        </expression>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_subroutine_dec_function_void_with_params() {
    let tokens = vec![
        Token { kind: TokenType::FUNCTION, lexeme: "function".to_string(), line: 1 },
        Token { kind: TokenType::VOID, lexeme: "void".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "main".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "b".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 3 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 4 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_subroutine_dec().unwrap();
    let expected = r#"<subroutineDec>
  <keyword> function </keyword>
  <keyword> void </keyword>
  <identifier> main </identifier>
  <symbol> ( </symbol>
  <parameterList>
    <keyword> int </keyword>
    <identifier> a </identifier>
    <symbol> , </symbol>
    <keyword> int </keyword>
    <identifier> b </identifier>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <statements>
      <returnStatement>
        <keyword> return </keyword>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_subroutine_dec_method_with_user_type_return() {
    let tokens = vec![
        Token { kind: TokenType::METHOD, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Square".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "getSquare".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "Square".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 3 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "Square".to_string(), line: 3 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "new".to_string(), line: 3 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 3 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 3 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 3 },
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 4 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 4 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 5 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 6 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_subroutine_dec().unwrap();
    let expected = r#"<subroutineDec>
  <keyword> method </keyword>
  <identifier> Square </identifier>
  <identifier> getSquare </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <varDec>
      <keyword> var </keyword>
      <identifier> Square </identifier>
      <identifier> s </identifier>
      <symbol> ; </symbol>
    </varDec>
    <statements>
      <letStatement>
        <keyword> let </keyword>
        <identifier> s </identifier>
        <symbol> = </symbol>
        <expression>
          <term>
            <identifier> Square </identifier>
            <symbol> . </symbol>
            <identifier> new </identifier>
            <symbol> ( </symbol>
            <expressionList>
            </expressionList>
            <symbol> ) </symbol>
          </term>
        </expression>
        <symbol> ; </symbol>
      </letStatement>
      <returnStatement>
        <keyword> return </keyword>
        <expression>
          <term>
            <identifier> s </identifier>
          </term>
        </expression>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_subroutine_dec_function_int_with_mixed_params() {
    let tokens = vec![
        Token { kind: TokenType::FUNCTION, lexeme: "function".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "sum".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 1 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Array".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "arr".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 2 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "total".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "total".to_string(), line: 3 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "x".to_string(), line: 3 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "arr".to_string(), line: 3 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 3 },
        Token { kind: TokenType::NUMBER, lexeme: "0".to_string(), line: 3 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 3 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 3 },
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "total".to_string(), line: 4 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 4 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 5 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 6 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_subroutine_dec().unwrap();
    let expected = r#"<subroutineDec>
  <keyword> function </keyword>
  <keyword> int </keyword>
  <identifier> sum </identifier>
  <symbol> ( </symbol>
  <parameterList>
    <keyword> int </keyword>
    <identifier> x </identifier>
    <symbol> , </symbol>
    <identifier> Array </identifier>
    <identifier> arr </identifier>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <varDec>
      <keyword> var </keyword>
      <keyword> int </keyword>
      <identifier> total </identifier>
      <symbol> ; </symbol>
    </varDec>
    <statements>
      <letStatement>
        <keyword> let </keyword>
        <identifier> total </identifier>
        <symbol> = </symbol>
        <expression>
          <term>
            <identifier> x </identifier>
          </term>
          <symbol> + </symbol>
          <term>
            <identifier> arr </identifier>
            <symbol> [ </symbol>
            <expression>
              <term>
                <integerConstant> 0 </integerConstant>
              </term>
            </expression>
            <symbol> ] </symbol>
          </term>
        </expression>
        <symbol> ; </symbol>
      </letStatement>
      <returnStatement>
        <keyword> return </keyword>
        <expression>
          <term>
            <identifier> total </identifier>
          </term>
        </expression>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_subroutine_dec_method_void_no_params_complex_body() {
    let tokens = vec![
        Token { kind: TokenType::METHOD, lexeme: "method".to_string(), line: 1 },
        Token { kind: TokenType::VOID, lexeme: "void".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "update".to_string(), line: 1 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 1 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 2 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 3 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 3 },
        Token { kind: TokenType::NUMBER, lexeme: "0".to_string(), line: 3 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 3 },
        Token { kind: TokenType::WHILE, lexeme: "while".to_string(), line: 4 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 4 },
        Token { kind: TokenType::LT, lexeme: "<".to_string(), line: 4 },
        Token { kind: TokenType::NUMBER, lexeme: "10".to_string(), line: 4 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 4 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 4 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 5 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 5 },
        Token { kind: TokenType::PLUS, lexeme: "+".to_string(), line: 5 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 5 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 5 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 6 },
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 7 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 7 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 8 },
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 9 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_subroutine_dec().unwrap();
    let expected = r#"<subroutineDec>
  <keyword> method </keyword>
  <keyword> void </keyword>
  <identifier> update </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <varDec>
      <keyword> var </keyword>
      <keyword> int </keyword>
      <identifier> i </identifier>
      <symbol> ; </symbol>
    </varDec>
    <statements>
      <letStatement>
        <keyword> let </keyword>
        <identifier> i </identifier>
        <symbol> = </symbol>
        <expression>
          <term>
            <integerConstant> 0 </integerConstant>
          </term>
        </expression>
        <symbol> ; </symbol>
      </letStatement>
      <whileStatement>
        <keyword> while </keyword>
        <symbol> ( </symbol>
        <expression>
          <term>
            <identifier> i </identifier>
          </term>
          <symbol> &lt; </symbol>
          <term>
            <integerConstant> 10 </integerConstant>
          </term>
        </expression>
        <symbol> ) </symbol>
        <symbol> { </symbol>
        <statements>
          <letStatement>
            <keyword> let </keyword>
            <identifier> i </identifier>
            <symbol> = </symbol>
            <expression>
              <term>
                <identifier> i </identifier>
              </term>
              <symbol> + </symbol>
              <term>
                <integerConstant> 1 </integerConstant>
              </term>
            </expression>
            <symbol> ; </symbol>
          </letStatement>
        </statements>
        <symbol> } </symbol>
      </whileStatement>
      <returnStatement>
        <keyword> return </keyword>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>"#;
    assert_eq!(parser.get_xml(), expected);
}

#[test]
fn test_parse_class_main() {
    let tokens = vec![
        // class Main {
        Token { kind: TokenType::CLASS, lexeme: "class".to_string(), line: 1 },
        Token { kind: TokenType::IDENT, lexeme: "Main".to_string(), line: 1 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 1 },
        // static boolean test;
        Token { kind: TokenType::STATIC, lexeme: "static".to_string(), line: 2 },
        Token { kind: TokenType::BOOLEAN, lexeme: "boolean".to_string(), line: 2 },
        Token { kind: TokenType::IDENT, lexeme: "test".to_string(), line: 2 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 2 },
        // function void main() { ... }
        Token { kind: TokenType::FUNCTION, lexeme: "function".to_string(), line: 3 },
        Token { kind: TokenType::VOID, lexeme: "void".to_string(), line: 3 },
        Token { kind: TokenType::IDENT, lexeme: "main".to_string(), line: 3 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 3 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 3 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 3 },
        // var SquareGame game;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "SquareGame".to_string(), line: 4 },
        Token { kind: TokenType::IDENT, lexeme: "game".to_string(), line: 4 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 4 },
        // let game = SquareGame.new();
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "game".to_string(), line: 5 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "SquareGame".to_string(), line: 5 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 5 },
        Token { kind: TokenType::IDENT, lexeme: "new".to_string(), line: 5 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 5 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 5 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 5 },
        // do game.run();
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 6 },
        Token { kind: TokenType::IDENT, lexeme: "game".to_string(), line: 6 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 6 },
        Token { kind: TokenType::IDENT, lexeme: "run".to_string(), line: 6 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 6 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 6 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 6 },
        // do game.dispose();
        Token { kind: TokenType::DO, lexeme: "do".to_string(), line: 7 },
        Token { kind: TokenType::IDENT, lexeme: "game".to_string(), line: 7 },
        Token { kind: TokenType::DOT, lexeme: ".".to_string(), line: 7 },
        Token { kind: TokenType::IDENT, lexeme: "dispose".to_string(), line: 7 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 7 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 7 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 7 },
        // return;
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 8 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 8 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 9 },
        // function void more() { ... }
        Token { kind: TokenType::FUNCTION, lexeme: "function".to_string(), line: 10 },
        Token { kind: TokenType::VOID, lexeme: "void".to_string(), line: 10 },
        Token { kind: TokenType::IDENT, lexeme: "more".to_string(), line: 10 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 10 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 10 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 10 },
        // var int i, j;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 11 },
        Token { kind: TokenType::INT, lexeme: "int".to_string(), line: 11 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 11 },
        Token { kind: TokenType::COMMA, lexeme: ",".to_string(), line: 11 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 11 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 11 },
        // var String s;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 12 },
        Token { kind: TokenType::IDENT, lexeme: "String".to_string(), line: 12 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 12 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 12 },
        // var Array a;
        Token { kind: TokenType::VAR, lexeme: "var".to_string(), line: 13 },
        Token { kind: TokenType::IDENT, lexeme: "Array".to_string(), line: 13 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 13 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 13 },
        // if (false) { ... } else { ... } (abreviado para o teste)
        // O conteúdo completo do if/else pode ser simplificado ou mantido.
        // Vou incluir o if/else completo conforme o Main.xml original
        Token { kind: TokenType::IF, lexeme: "if".to_string(), line: 14 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 14 },
        Token { kind: TokenType::FALSE, lexeme: "false".to_string(), line: 14 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 14 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 14 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 15 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 15 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 15 },
        Token { kind: TokenType::STRING, lexeme: "string constant".to_string(), line: 15 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 15 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 16 },
        Token { kind: TokenType::IDENT, lexeme: "s".to_string(), line: 16 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 16 },
        Token { kind: TokenType::NULL, lexeme: "null".to_string(), line: 16 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 16 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 17 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 17 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 17 },
        Token { kind: TokenType::NUMBER, lexeme: "1".to_string(), line: 17 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 17 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 17 },
        Token { kind: TokenType::IDENT, lexeme: "a".to_string(), line: 17 },
        Token { kind: TokenType::LBRACKET, lexeme: "[".to_string(), line: 17 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 17 },
        Token { kind: TokenType::RBRACKET, lexeme: "]".to_string(), line: 17 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 17 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 18 },
        Token { kind: TokenType::ELSE, lexeme: "else".to_string(), line: 18 },
        Token { kind: TokenType::LBRACE, lexeme: "{".to_string(), line: 18 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 19 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 19 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 19 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 19 },
        Token { kind: TokenType::ASTERISK, lexeme: "*".to_string(), line: 19 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 19 },
        Token { kind: TokenType::MINUS, lexeme: "-".to_string(), line: 19 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 19 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 19 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 19 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 20 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 20 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 20 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 20 },
        Token { kind: TokenType::SLASH, lexeme: "/".to_string(), line: 20 },
        Token { kind: TokenType::LPAREN, lexeme: "(".to_string(), line: 20 },
        Token { kind: TokenType::MINUS, lexeme: "-".to_string(), line: 20 },
        Token { kind: TokenType::NUMBER, lexeme: "2".to_string(), line: 20 },
        Token { kind: TokenType::RPAREN, lexeme: ")".to_string(), line: 20 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 20 },
        Token { kind: TokenType::LET, lexeme: "let".to_string(), line: 21 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 21 },
        Token { kind: TokenType::EQ, lexeme: "=".to_string(), line: 21 },
        Token { kind: TokenType::IDENT, lexeme: "i".to_string(), line: 21 },
        Token { kind: TokenType::OR, lexeme: "|".to_string(), line: 21 },
        Token { kind: TokenType::IDENT, lexeme: "j".to_string(), line: 21 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 21 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 22 },
        Token { kind: TokenType::RETURN, lexeme: "return".to_string(), line: 23 },
        Token { kind: TokenType::SEMICOLON, lexeme: ";".to_string(), line: 23 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 24 },
        Token { kind: TokenType::RBRACE, lexeme: "}".to_string(), line: 25 }, // class closing brace
        Token { kind: TokenType::EOF, lexeme: "".to_string(), line: 26 },
    ];
    let mut parser = Parser::new(tokens);
    parser.parse_code().unwrap();

    let expected = r#"<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <classVarDec>
    <keyword> static </keyword>
    <keyword> boolean </keyword>
    <identifier> test </identifier>
    <symbol> ; </symbol>
  </classVarDec>
  <subroutineDec>
    <keyword> function </keyword>
    <keyword> void </keyword>
    <identifier> main </identifier>
    <symbol> ( </symbol>
    <parameterList>
    </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
      <symbol> { </symbol>
      <varDec>
        <keyword> var </keyword>
        <identifier> SquareGame </identifier>
        <identifier> game </identifier>
        <symbol> ; </symbol>
      </varDec>
      <statements>
        <letStatement>
          <keyword> let </keyword>
          <identifier> game </identifier>
          <symbol> = </symbol>
          <expression>
            <term>
              <identifier> SquareGame </identifier>
              <symbol> . </symbol>
              <identifier> new </identifier>
              <symbol> ( </symbol>
              <expressionList>
              </expressionList>
              <symbol> ) </symbol>
            </term>
          </expression>
          <symbol> ; </symbol>
        </letStatement>
        <doStatement>
          <keyword> do </keyword>
          <identifier> game </identifier>
          <symbol> . </symbol>
          <identifier> run </identifier>
          <symbol> ( </symbol>
          <expressionList>
          </expressionList>
          <symbol> ) </symbol>
          <symbol> ; </symbol>
        </doStatement>
        <doStatement>
          <keyword> do </keyword>
          <identifier> game </identifier>
          <symbol> . </symbol>
          <identifier> dispose </identifier>
          <symbol> ( </symbol>
          <expressionList>
          </expressionList>
          <symbol> ) </symbol>
          <symbol> ; </symbol>
        </doStatement>
        <returnStatement>
          <keyword> return </keyword>
          <symbol> ; </symbol>
        </returnStatement>
      </statements>
      <symbol> } </symbol>
    </subroutineBody>
  </subroutineDec>
  <subroutineDec>
    <keyword> function </keyword>
    <keyword> void </keyword>
    <identifier> more </identifier>
    <symbol> ( </symbol>
    <parameterList>
    </parameterList>
    <symbol> ) </symbol>
    <subroutineBody>
      <symbol> { </symbol>
      <varDec>
        <keyword> var </keyword>
        <keyword> int </keyword>
        <identifier> i </identifier>
        <symbol> , </symbol>
        <identifier> j </identifier>
        <symbol> ; </symbol>
      </varDec>
      <varDec>
        <keyword> var </keyword>
        <identifier> String </identifier>
        <identifier> s </identifier>
        <symbol> ; </symbol>
      </varDec>
      <varDec>
        <keyword> var </keyword>
        <identifier> Array </identifier>
        <identifier> a </identifier>
        <symbol> ; </symbol>
      </varDec>
      <statements>
        <ifStatement>
          <keyword> if </keyword>
          <symbol> ( </symbol>
          <expression>
            <term>
              <keyword> false </keyword>
            </term>
          </expression>
          <symbol> ) </symbol>
          <symbol> { </symbol>
          <statements>
            <letStatement>
              <keyword> let </keyword>
              <identifier> s </identifier>
              <symbol> = </symbol>
              <expression>
                <term>
                  <stringConstant> string constant </stringConstant>
                </term>
              </expression>
              <symbol> ; </symbol>
            </letStatement>
            <letStatement>
              <keyword> let </keyword>
              <identifier> s </identifier>
              <symbol> = </symbol>
              <expression>
                <term>
                  <keyword> null </keyword>
                </term>
              </expression>
              <symbol> ; </symbol>
            </letStatement>
            <letStatement>
              <keyword> let </keyword>
              <identifier> a </identifier>
              <symbol> [ </symbol>
              <expression>
                <term>
                  <integerConstant> 1 </integerConstant>
                </term>
              </expression>
              <symbol> ] </symbol>
              <symbol> = </symbol>
              <expression>
                <term>
                  <identifier> a </identifier>
                  <symbol> [ </symbol>
                  <expression>
                    <term>
                      <integerConstant> 2 </integerConstant>
                    </term>
                  </expression>
                  <symbol> ] </symbol>
                </term>
              </expression>
              <symbol> ; </symbol>
            </letStatement>
          </statements>
          <symbol> } </symbol>
          <keyword> else </keyword>
          <symbol> { </symbol>
          <statements>
            <letStatement>
              <keyword> let </keyword>
              <identifier> i </identifier>
              <symbol> = </symbol>
              <expression>
                <term>
                  <identifier> i </identifier>
                </term>
                <symbol> * </symbol>
                <term>
                  <symbol> ( </symbol>
                  <expression>
                    <term>
                      <symbol> - </symbol>
                      <term>
                        <identifier> j </identifier>
                      </term>
                    </term>
                  </expression>
                  <symbol> ) </symbol>
                </term>
              </expression>
              <symbol> ; </symbol>
            </letStatement>
            <letStatement>
              <keyword> let </keyword>
              <identifier> j </identifier>
              <symbol> = </symbol>
              <expression>
                <term>
                  <identifier> j </identifier>
                </term>
                <symbol> / </symbol>
                <term>
                  <symbol> ( </symbol>
                  <expression>
                    <term>
                      <symbol> - </symbol>
                      <term>
                        <integerConstant> 2 </integerConstant>
                      </term>
                    </term>
                  </expression>
                  <symbol> ) </symbol>
                </term>
              </expression>
              <symbol> ; </symbol>
            </letStatement>
            <letStatement>
              <keyword> let </keyword>
              <identifier> i </identifier>
              <symbol> = </symbol>
              <expression>
                <term>
                  <identifier> i </identifier>
                </term>
                <symbol> | </symbol>
                <term>
                  <identifier> j </identifier>
                </term>
              </expression>
              <symbol> ; </symbol>
            </letStatement>
          </statements>
          <symbol> } </symbol>
        </ifStatement>
        <returnStatement>
          <keyword> return </keyword>
          <symbol> ; </symbol>
        </returnStatement>
      </statements>
      <symbol> } </symbol>
    </subroutineBody>
  </subroutineDec>
  <symbol> } </symbol>
</class>"#;

    assert_eq!(parser.get_xml(), expected);
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
                arquivos_referencia.push(file_str.replace(".jack", ".xml"));
            }
        }
        
        for i in 0..arquivos_jack.len(){
            let path_jack = format!("{}/{}",pasta, arquivos_jack[i]);
            let path_referencia = format!("{}/{}",pasta, arquivos_referencia[i]);
            
            let code =fs::read_to_string(path_jack).expect("Falha ao ler arquivo");
            let mut xml_referencia = fs::read_to_string(path_referencia).expect("Falha ao ler arquivo");
            let mut scanner = Scanner::new(code);
            let tokens = scanner.tokenize().clone();
            let mut parser = Parser::new(tokens);
            parser.parse_code().unwrap();
            let mut xml_gerado = parser.get_xml();

            xml_gerado = xml_gerado.replace("\r\n", "\n").trim_end().to_string();
            xml_referencia = xml_referencia.replace("\r\n", "\n").trim_end().to_string();
           
            assert_eq!(xml_gerado, xml_referencia);
            
        }
    }

}
