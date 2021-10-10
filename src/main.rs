use std::env;
use std::error::Error;
use std::process;

use rust_chess::library::config::*;
use rust_chess::library::cli_chess;
use rust_chess::library::bevy_chess;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to prepare config: {}", err);
        process::exit(1);
    });
    
    println!("Running the {} version...", config.gui_type);
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(),Box<dyn Error>> {
    match config.gui_type {
        GuiType::CLI => cli_chess::run(), 
        GuiType::Bevy => bevy_chess::run()
    }
    Ok(())
}