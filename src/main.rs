use std::{env, error::Error, process};
use rust_chess::library::{config::*, cli, gui};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to prepare config: {}", err);
        process::exit(1);
    });
    
    println!("Running the {} version...", config.ui_type);
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(),Box<dyn Error>> {
    match config.ui_type {
        UiType::CLI => cli::run(config.fen), 
        UiType::GUI => unsafe{ gui::run(config.fen) }
    }
    Ok(())
}