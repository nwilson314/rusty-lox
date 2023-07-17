use std::{fs, io};
use std::io::{Write, stdout};

use crate::error::Error;
use crate::scanner::Scanner;

pub struct Lox {
    error: Error
}

impl Lox {
    pub fn new() -> Lox {
        let error = Error::new();
        Lox {
            error: error
        }
    }

    pub fn run_file(&mut self, path: &str) {
        println!("Running file: {}", path);
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");

        self.run(&contents);
        if self.error.had_error {
            std::process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        println!("Running prompt");
        loop {
            let mut buffer = String::new();
            print!("> ");
            stdout().flush().unwrap();
            let res = io::stdin().read_line(&mut buffer);
            match res{
                Ok(0) => break,
                Ok(_) => {
                    self.run(buffer.as_str());
                    self.error.had_error = false;
                }
                _ => break
            }
        }
        
    }

    fn run(&mut self, source: &str) {
        println!("Running source: {}", source);
        let mut scanner = Scanner::new(source.to_string(), &mut self.error);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }

        if self.error.had_error {
            std::process::exit(65);
        }   
    }
}