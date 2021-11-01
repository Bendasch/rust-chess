use std::env;
use std::fmt;

pub enum GuiType {
    CLI,
    OPENGL
}

impl fmt::Display for GuiType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GuiType::CLI => write!(f, "command line"),
            GuiType::OPENGL => write!(f, "OpenGL")
        } 
    }    
}
pub struct Config {
    pub gui_type: GuiType,
    pub fen: Option<String>
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next(); // skip the program name

        let gui_type = match args.next() {
            Some(arg) if arg == "cli" => GuiType::CLI,
            Some(arg) if arg == "opengl" => GuiType::OPENGL,
            Some(_) => return Err("Please enter a valid GUI type (cli / bevy)."), 
            None => return Err("Please enter a valid GUI type (cli / bevy).")
        };

        let fen = args.next();

        Ok(Config { gui_type, fen })
    }
}