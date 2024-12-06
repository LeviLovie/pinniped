pub mod args;
pub mod engine;
pub mod included_libs;
#[cfg(test)]
mod test;
pub mod tokens;

use log::{error, info};

use crate::engine::{file::File, machine::Machine};
use crate::{included_libs::libs, tokens::tokens};

fn main() {
    pretty_env_logger::init();
    let mut args = args::parse_args();
    if args.file == "NONE" {
        if args.args.is_empty() {
            error!("No file or arguments provided");
            std::process::exit(1);
        }
        args.file = args.args[0].clone();
    }
    info!("Starting interpreter on file: {}", args.file);

    info!("Loading main file: {}", args.file);
    let mut main_file = match File::new("main".to_string(), args.file.clone()) {
        Ok(file) => file,
        Err(e) => {
            error!("Error loading main file: {}", e);
            std::process::exit(1);
        }
    };
    match main_file.read() {
        Ok(_) => {}
        Err(e) => {
            error!("Error reading main file: {}", e);
            std::process::exit(1);
        }
    };
    info!("Main file loaded");

    let mut machine = Machine::new(args, main_file);
    info!("Machine created");

    machine.register_tokens(tokens());
    machine.register_libs(libs());

    match machine.preprocess() {
        Ok(_) => {}
        Err(e) => {
            error!("Error during preprocessing: {}", e);
            std::process::exit(1);
        }
    };

    match machine.lex() {
        Ok(_) => {}
        Err(e) => {
            error!("Error during lexing: {}", e);
            std::process::exit(1);
        }
    };

    match machine.interpret() {
        Ok(_) => {}
        Err(e) => {
            error!("Error during interpretation: {}", e);
            std::process::exit(1);
        }
    };
}
