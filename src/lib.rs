use std::fs::{File, OpenOptions};
use std::io::stdin;
use std::error;
use colored::*;
use std::io::{self, Read, Write, BufWriter};

pub struct Config{
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 2{
            return Err("not enough arguments!!"); 
        }
        let filename: String = args[1].clone();
        Ok(Config{filename})
    }
}

pub fn open_and_read_file(config: &Config) -> String {
    let f = File::open(&config.filename);

    let mut file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                match File::create(&config.filename) {
                    Ok(_) => return String::new(), 
                    Err(error) => panic!("Error creating the file: {:?}", error),
                }
            }
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => content,
        Err(error) => panic!("Error reading the file content: {:?}", error),
    }
}

pub fn content(config: &Config) -> Result<String, Box<dyn error::Error>>{
   let content = open_and_read_file(&config);
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
pub fn save_to_file(config: &Config, content: &String) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&config.filename)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    Ok(())
}

