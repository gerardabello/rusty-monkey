use super::{Lexer, Token};

fn lex_string(code: &str) -> Vec<Token> {
    Lexer::new(code.chars()).collect()
}

#[test]
fn test_single_char_tokens() {

    let code = "
        * / !
        ,
        ;
        =
        {}()
        '\"
        >
        <
        ";

    let expected_tokens = vec![
        Token::Asterisk,
        Token::Slash,
        Token::Bang,
        Token::Comma,
        Token::Semicolon,
        Token::Assign,
        Token::OpenBrace,
        Token::CloseBrace,
        Token::OpenParenthesis,
        Token::CloseParenthesis,
        Token::SingleQuote,
        Token::DoubleQuote,
        Token::GreaterThan,
        Token::LessThan,
    ];

    assert_eq!(lex_string(code), expected_tokens);

}

#[test]
fn test_multiple_char_tokens() {

    let code = "
        ==
        !=
        ! =
           = =
           >= <=
        ";

    let expected_tokens = vec![
        Token::Equal,
        Token::NotEqual,
        Token::Bang,
        Token::Assign,
        Token::Assign,
        Token::Assign,
        Token::GreaterThanEqual,
        Token::LessThanEqual,
    ];

    assert_eq!(lex_string(code), expected_tokens);
}

#[test]
fn test_number_tokens() {

    let code = "
        0 9
        09
        50 3336 0005
        ";

    let expected_tokens = vec![
        Token::Integer{string: String::from("0")},
        Token::Integer{string: String::from("9")},
        Token::Integer{string: String::from("09")},
        Token::Integer{string: String::from("50")},
        Token::Integer{string: String::from("3336")},
        Token::Integer{string: String::from("0005")},
    ];

    assert_eq!(lex_string(code), expected_tokens);
}

#[test]
fn test_word_tokens() {

    let code = "
            let if else fn return true false
            Animal dog Cat mandarinA
        ";

    let expected_tokens = vec![
        Token::Let,
        Token::If,
        Token::Else,
        Token::Function,
        Token::Return,
        Token::True,
        Token::False,
        Token::Identifier{name: String::from("Animal")},
        Token::Identifier{name: String::from("dog")},
        Token::Identifier{name: String::from("Cat")},
        Token::Identifier{name: String::from("mandarinA")},
    ];

    assert_eq!(lex_string(code), expected_tokens);
}

#[test]
fn test_code_sum_function() {

    let code = "
        fn calculation (a, b) {
          return a + (b-42 );
        }
        ";

    let expected_tokens = vec![
        Token::Function,
        Token::Identifier { name: String::from("calculation")},
        Token::OpenParenthesis,
        Token::Identifier { name: String::from("a")},
        Token::Comma ,
        Token::Identifier { name: String::from("b")},
        Token::CloseParenthesis,
        Token::OpenBrace,
        Token::Return,
        Token::Identifier { name: String::from("a")},
        Token::Plus,
        Token::OpenParenthesis,
        Token::Identifier { name: String::from("b")},
        Token::Minus,
        Token::Integer { string: String::from("42")},
        Token::CloseParenthesis,
        Token::Semicolon,
        Token::CloseBrace,
    ];

    assert_eq!(lex_string(code), expected_tokens);

}
