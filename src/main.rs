use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::token::Token;

mod lexer;
mod token;

fn repl_start() -> Result<(), std::io::Error> {
    let stdin = io::stdin();

'main_loop:
    loop {
        print!(">>> ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        // pop the \r, \n, \r\n
        while buffer.ends_with('\n') || buffer.ends_with('\r') {
            buffer.pop();
        }


        if buffer.eq("") {
            break 'main_loop Ok(());
        }

        let mut lexer = Lexer::new(buffer);
        
        // print all the tokens
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            if token == Token::Eof {
                break;
            }
        }
    }
}

fn main() {
    match repl_start() {
        Ok(()) => (),
        Err(why) => println!("Could not read the input: {:?}", why),
    }
}
