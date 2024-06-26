
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

mod lexer;
mod parser;


use parser::Parser;




/// main
fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
    let mut p = Parser::new(input);
    let ast = p.parse()?;
    match ast.eval(env) {
        Some(result) => Ok(result),
        None => Err("No value for that expression!".to_string())
    }
}


fn main() {
    use std::f64;
    let mut env = HashMap::new();
    //在符号表中预定义一些变量
    env.insert("wow".to_string(), 35.0f64);
    env.insert("pi".to_string(), f64::consts::PI);

    let stdin = io::stdin();


    loop {
        print!(">> ");
        io::stdout().flush().ok();

        let mut input = String::new();

        //从标准输入读一行写入input
        match stdin.read_line(&mut input) {
            Ok(_) => {

                if input.len() == 0 {
                    println!("");
                    return;
                }

                let expression_text = input.trim_end();

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