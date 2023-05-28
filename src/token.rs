use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token{
    Illegal(String),
    Eof,
    // Identifiers + literals
    Ident(String),     
    Int(String),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Slash,
    Asterisk,
    Eq,
    NotEq,
    
    Lt,
    Gt,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

pub struct Keywords{
    map: HashMap<String, Token>,
}

impl Keywords {
    pub fn new() -> Keywords{
        let mut keywords = Keywords { map: HashMap::new()};

        // set all the keywords in the language
        keywords.insert(String::from("let"),    Token::Let);
        keywords.insert(String::from("fn"),     Token::Function);
        keywords.insert(String::from("if"),     Token::If);
        keywords.insert(String::from("else"),   Token::Else);
        keywords.insert(String::from("return"), Token::Return);
        keywords.insert(String::from("true"),   Token::True);
        keywords.insert(String::from("false"),  Token::False);

        //return
        keywords
    }

    pub fn insert(&mut self, key: String, token: Token){
        self.map.insert(key, token);
    }
    
    
    pub fn look_up_ident(&self, ident: &String) -> Token {
        match self.map.get(ident).clone(){
            Some(t) => t.clone(),
            None => Token::Ident(ident.to_string()),
        }
    }

}

#[cfg(test)]
mod test{
    use super::Keywords;
    use crate::token::Token;
    #[test]
    fn test_look_up_ident(){
        let keywords = Keywords::new();
        let test_ident = vec![("let", Token::Let), ("fn", Token::Function), ("five", Token::Ident("five".to_string()))];
        for ident in test_ident{
            let (literal, token) = ident;
            assert_eq!(keywords.look_up_ident(&literal.to_string()), token);
        }
    }
}
