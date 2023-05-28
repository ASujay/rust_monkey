use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Token{
    Illegal(String),
    Eof,
    // Identifiers + literals
    Ident(String),     
    Int(String),
    // Operators
    Assign,
    Plus,
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
}

pub struct Keywords{
    map: HashMap<String, Token>,
}

impl Keywords {
    pub fn new() -> Keywords{
        let mut keywords = Keywords { map: HashMap::new()};

        // set all the keywords in the language
        keywords.insert(String::from("let"), Token::Let);
        keywords.insert(String::from("fn"), Token::Function);

        //return
        keywords
    }

    pub fn insert(&mut self, key: String, token: Token){
        self.map.insert(key, token);
    }

    pub fn 

}
