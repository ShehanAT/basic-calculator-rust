use std::f32::consts::PI;
use std::f64::consts::PI;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
// pub makes an item visible within the provided path. 
pub mod parser;


fn evaluate(input: &str, env: &mut HashMap<String f64>) -> Result<f64, String> {
    let mut p = parser::Parser::new(input);
    let ast = p.parse()?;
    match ast.eval(env) {
        Some(result) => Ok(result),
        None => Err("No value for that expression!".to_string())
    }
}


pub fn main() {
    use std::f64;
    let mut env = HashMap::new();
    env.insert("wow".to_string(), 35.0f64);
    env.insert("pi".to_string(), f64::const::PI);

    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().ok();

        let mut input = String::new();

        // the below match block is a error handling mechanism with the Result type. Ok() representing success, Err() representing error and containing an error error
        match stdin.read_line(&mut input) {
            Ok(_) => {
                if input.len() == 0 {
                    println!("");
                    return ;
                }

                let expression_text = input.trim_right();

                let result = evaluate(expression_text, &mut env);
                match result {
                    Ok(value) => {
                        println!("=> {}", value);
                    }
                    Err(s) => {
                        println!("Error: {}", s);
                    }
                }
                io::stdout().flush().ok();
            }
            Err(_) => {
                println!("");
                return;
            }
        }
    }
}


