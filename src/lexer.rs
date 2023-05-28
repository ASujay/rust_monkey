use crate::token::Token;

pub struct Lexer{ 
    position: usize, 
    read_position: usize,
    ch: u8,
    input_as_bytes: Vec<u8>,
}

impl Lexer{
    pub fn new(input: String) -> Lexer{
        let mut lexer = Lexer{
            position: 0,
            read_position: 0,
            ch: 0,
            input_as_bytes: input.as_bytes().to_vec(),
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

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic(){
            self.read_char();
        }
        return std::str::from_utf8(&self.input_as_bytes[position..=self.position]).unwrap().to_string();
    }

    pub fn look_up_ident(ident: String) -> Token{
        match &ident {
            "let" => Token::Let,
            "fn"  => Token::Function,
            _     => Token::Ident(ident.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token{
        let token = match self.ch {
            b'=' => Token::Assign,
            b',' => Token::Comma,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            0    => Token::Eof,
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let literal = self.read_identifier();
                    // check if the literal collected is a keyword or not

                } else {
                    Token::Illegal(String::from(self.ch))
                }
            }       
        };
        self.read_char();
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
            Token::Eof,
        ];

        // token initialization
        let mut lexer = Lexer::new(input.to_string());

        // iterate over the tests vec and check each token
        for test in tests{
            let tok = lexer.next_token();
            assert_eq!(tok, test);
        }
    }
}
