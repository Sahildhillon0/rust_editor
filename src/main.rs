use std::{env, process};
use rust_editor::{self, Config};
use std::error;
use colored::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    
    let _args: Vec<String> = env::args().collect();
    
    let config = Config::new(&_args).unwrap_or_else(|err|{
        eprintln!("Problem parsing the Arguments : {}",err.red());
        process::exit(1);
    });
    
    let new_content = rust_editor::content(&config)?;

    rust_editor::save_to_file(&config, &new_content).unwrap_or_else(|_err|{
        eprint!("error saving the file: {}",_err);
        process::exit(1);
    });

    println!("\n{}","File saved successfully!".green());
    Ok(())
}

