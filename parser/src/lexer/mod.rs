use std::iter::Iterator;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier { name: String },
    Integer { string: String },
    StringLiteral { string: String },
    Let,
    If,
    Else,
    Function,
    Assign,
    Return,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,

    True,
    False,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenSquare,
    CloseSquare,

    Comma,
    Semicolon,
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic()
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

pub struct Lexer<T: Iterator<Item = char>> {
    iter: T,
    saved_char: Option<char>,
}

impl<T: Iterator<Item = char>> Lexer<T> {
    pub fn new(iter: T) -> Self {
        Lexer {
            iter,
            saved_char: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.saved_char {
            self.saved_char = None;
            return Some(c);
        }
        if let Some(c) = self.iter.next() {
            return Some(c);
        }
        None
    }

    fn save_char_for_next_loop(&mut self, c: char) {
        if !is_whitespace(c) {
            self.saved_char = Some(c);
        }
    }

    fn next_char_skipping_whitespace(&mut self) -> Option<char> {
        if let Some(c) = self.next_char() {
            if is_whitespace(c) {
                return self.next_char_skipping_whitespace();
            }
            return Some(c);
        }
        None
    }

    fn next_token_starting_with_equal(&mut self) -> Option<Token> {
        if let Some(c) = self.next_char() {
            match c {
                '=' => Some(Token::Equal),
                remain => {
                    self.save_char_for_next_loop(remain);
                    Some(Token::Assign)
                }
            }
        } else {
            None
        }
    }

    fn next_token_starting_with_bang(&mut self) -> Option<Token> {
        if let Some(c) = self.next_char() {
            match c {
                '=' => Some(Token::NotEqual),
                remain => {
                    self.save_char_for_next_loop(remain);
                    Some(Token::Bang)
                }
            }
        } else {
            None
        }
    }

    fn next_token_starting_with_less_than(&mut self) -> Option<Token> {
        if let Some(c) = self.next_char() {
            match c {
                '=' => Some(Token::LessThanEqual),
                remain => {
                    self.save_char_for_next_loop(remain);
                    Some(Token::LessThan)
                }
            }
        } else {
            None
        }
    }

    fn next_token_starting_with_greater_than(&mut self) -> Option<Token> {
        if let Some(c) = self.next_char() {
            match c {
                '=' => Some(Token::GreaterThanEqual),
                remain => {
                    self.save_char_for_next_loop(remain);
                    Some(Token::GreaterThan)
                }
            }
        } else {
            None
        }
    }

    fn next_number_token(&mut self, first: char) -> Token {
        let mut string = first.to_string();
        loop {
            if let Some(c) = self.next_char() {
                if is_number(c) {
                    string.push(c);
                    continue;
                } else {
                    self.save_char_for_next_loop(c);
                }
            }
            break;
        }

        Token::Integer { string }
    }

    fn next_word_token(&mut self, first: char) -> Token {
        let mut string = first.to_string();
        loop {
            if let Some(c) = self.next_char() {
                if is_letter(c) {
                    string.push(c);
                    continue;
                } else {
                    self.save_char_for_next_loop(c);
                }
            }
            break;
        }

        match &string[..] {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "fn" => Token::Function,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier { name: string },
        }
    }

    fn next_string_literal_token(&mut self) -> Token {
        let mut string = String::new();
        loop {
            if let Some(c) = self.next_char() {
                if c != '"'{
                    string.push(c);
                    continue;
                } else {
                    break;
                }
            }
            break;
        }

        Token::StringLiteral { string }
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(c) = self.next_char_skipping_whitespace() {
            return match c {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Asterisk),
                '/' => Some(Token::Slash),
                ';' => Some(Token::Semicolon),
                '(' => Some(Token::OpenParenthesis),
                ')' => Some(Token::CloseParenthesis),
                '{' => Some(Token::OpenBrace),
                '}' => Some(Token::CloseBrace),
                '[' => Some(Token::OpenSquare),
                ']' => Some(Token::CloseSquare),
                ',' => Some(Token::Comma),
                '!' => self.next_token_starting_with_bang(),
                '=' => self.next_token_starting_with_equal(),
                '<' => self.next_token_starting_with_less_than(),
                '>' => self.next_token_starting_with_greater_than(),
                '"' => Some(self.next_string_literal_token()),
                c if is_letter(c) => Some(self.next_word_token(c)),
                c if is_number(c) => Some(self.next_number_token(c)),
                _ => panic!(format!("Unexpected character '{}'", c)),
            };
        }

        None
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
