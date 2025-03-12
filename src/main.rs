use std::{env, process};
use std::io::stdin;
use rust_editor::{self, Config};
use std::error;
use colored::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    
    let _args: Vec<String> = env::args().collect();
    
    let config = Config::new(&_args).unwrap_or_else(|err|{
        eprintln!("Problem parsing the Arguments : {}",err.red());
        process::exit(1);
    });
    
    let new_content = content(&config)?;

    rust_editor::save_to_file(&config, &new_content).unwrap_or_else(|_err|{
        eprint!("error saving the file: {}",_err);
        process::exit(1);
    });

    println!("\n{}","File saved successfully!".green());
    Ok(())
}

fn content(config: &Config) -> Result<String, Box<dyn error::Error>>{
   let content = rust_editor::open_and_read_file(&config);
    println!("{}","Current file content:\n".blue());
    println!("{}", content.green());
    
    println!("\n{}","Enter your new content (type 'END' on a new line to finish editing):".red());
    
    let mut new_content = String::new();
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        
        if line.trim() == "END" {
            break;
        }
        
        new_content.push_str(&line);
    }
    Ok(new_content)
}
