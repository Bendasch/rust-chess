use std::{env, fmt};

pub enum UiType {
    CLI,
    GUI
}

impl fmt::Display for UiType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UiType::CLI => write!(f, "command line"),
            UiType::GUI => write!(f, "graphical interface (OpenGL)")
        } 
    }    
}

pub struct Config {
    pub ui_type: UiType,
    pub fen: Option<String>
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next(); // skip the program name

        let ui_type = match args.next() {
            Some(arg) if arg == "cli" => UiType::CLI,
            Some(arg) if arg == "gui" => UiType::GUI,
            Some(_) => return Err("Please enter a valid UI type (cli / gui)."), 
            None => return Err("Please enter a valid UI type (cli / gui).")
        };

        let fen = args.next();

        Ok(Config { ui_type, fen })
    }
}