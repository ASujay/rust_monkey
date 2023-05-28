use crate::token::{Token, Keywords};

pub struct Lexer{ 
    position: usize, 
    read_position: usize,
    ch: u8,
    input_as_bytes: Vec<u8>,
    keywords: Keywords,
}

impl Lexer{
    pub fn new(input: String) -> Lexer{
        let mut lexer = Lexer{
            position: 0,
            read_position: 0,
            ch: 0,
            input_as_bytes: input.as_bytes().to_vec(),
            keywords: Keywords::new(),
        };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input_as_bytes.len() {
            self.ch = 0;
        } else {
            self.ch = self.input_as_bytes[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> u8{
        if self.read_position >= self.input_as_bytes.len() {
            return 0;
        } else {
            return self.input_as_bytes[self.read_position];
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic(){
            self.read_char();
        }
        return std::str::from_utf8(&self.input_as_bytes[position..self.position]).unwrap().to_string();
    }

    pub fn look_up_ident(&self, ident: &String) -> Token{
        self.keywords.look_up_ident(ident)
    }

    pub fn skip_whitespace(&mut self){
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> Token{
        let position = self.position;
        while self.ch.is_ascii_digit(){
            self.read_char();
        }
        return Token::Int(std::str::from_utf8(&self.input_as_bytes[position..self.position]).unwrap().to_string());
    }

    pub fn next_token(&mut self) -> Token{
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'='{
                    self.read_char();
                    self.read_char();
                    return Token::Eq;
                }
                self.read_char();
                Token::Assign
            },
            b',' => {
                self.read_char();
                Token::Comma
            },
            b'(' => { 
                self.read_char();
                Token::LParen
            },
            b')' => {
                self.read_char();
                Token::RParen
            },
            b'{' => {
                self.read_char();
                Token::LBrace
            },
            b'}' => {
                self.read_char();
                Token::RBrace
            },
            b';' => {
                self.read_char();
                Token::Semicolon
            },
            b'+' => {
                self.read_char();
                Token::Plus
            },
            b'-' => {
                self.read_char();
                Token::Minus
            },
            b'/' => {
                self.read_char();
                Token::Slash
            },
            b'*' => {
                self.read_char();
                Token::Asterisk
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    return Token::NotEq;
                }
                self.read_char();
                Token::Bang
            },
            b'<' => {
                self.read_char();
                Token::Lt
            },
            b'>' => {
                self.read_char();
                Token::Gt
            },
            0    => Token::Eof,
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let literal = self.read_identifier();
                    // check if the literal collected is a keyword or not
                    self.look_up_ident(&literal)
                } else if self.ch.is_ascii_digit() {
                    self.read_number()
                } else {
                    let ch = self.ch;
                    self.read_char();
                    Token::Illegal(format!("Illegal character: {}", ch))
                }
            }       
        };
        return token;
    } 
}

/* Test */

#[cfg(test)]

mod tests{
    use crate::token::Token;
    use crate::lexer::Lexer;
    #[test]
    fn test_next_token(){
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y){
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
                5 < 10 > 5;
            if(5 < 10){
             return true;
            } else {
             return false;
            }

            10 == 10;
            10 != 9;
            ";
        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang, 
            Token::Minus, 
            Token::Slash, 
            Token::Asterisk, 
            Token::Int("5".to_string()), 
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::Gt,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".to_string()),
            Token::Eq,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEq,
            Token::Int("9".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        // token initialization
        let mut lexer = Lexer::new(input.to_string());

        // iterate over the tests vec and check each token
        for test in tests{
            let tok = lexer.next_token();
            assert_eq!(test, tok);
        }
    }
}
