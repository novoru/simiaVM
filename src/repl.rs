use crate::lexer::Lexer;
use crate::parser:: { Parser };
use crate::eval::{ eval };
use std::io::{ self, Write, stdin };


pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let lexer = Lexer::new(input);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program().unwrap();

                if parser.errors.len() != 0 {
                    print_parse_errors(parser.errors);
                    continue;
                }

                let evaluated = eval(program);
                println!("{}", evaluated.inspect());
                
            }
            Err(error) => println!("error: {}", error)
        }
    }
}

fn print_parse_errors(errors: Vec<String>) {
    for error in errors {
        println!("{}", error);
    }
}
