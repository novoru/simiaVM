use crate::lexier::Lexier;
use crate::parser:: { Parser };
use std::io::{ self, Write, stdin };


pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let lexier = Lexier::new(input);
                let mut parser = Parser::new(lexier);
                let program = parser.parse_program().unwrap();

                if parser.errors.len() != 0 {
                    print_parse_errors(parser.errors);
                    continue;
                }

                println!("{}", program.inspect());
                
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
