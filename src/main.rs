mod lexer;
mod token;
mod error;
mod position;

use std::io::Write;

use lexer::Lexer;

fn run<'a, 'b>(text: String, file_name: String) -> Result<Vec<token::Token>, error::Error>{
    let lexer = Lexer::new(text, file_name);
    let result = lexer.generate_tokens();

    result
}

fn main() {
    loop {
        print!("Yron >>> ");
        std::io::stdout().flush().unwrap();

        let mut line = "".to_string();
        std::io::stdin().read_line(&mut line).expect("Failed to read line!");

        let result = run(line.clone(), "<stdin>".to_string().clone());
        
        match result {
            Ok(tokens) => {
                for token in tokens {
                    println!("{}", token.to_string());
                }
            },

            Err(error) => println!("{}", error.to_string())
        }
    }
}