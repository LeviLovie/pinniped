pub mod args;
pub mod engine;
pub mod tokens;

use log::{error, info};

use crate::engine::machine::Machine;
use crate::tokens::tokens;

fn main() {
    pretty_env_logger::init();
    let args = args::parse_args();
    info!("Starting interpreter on file: {}", args.file);

    let mut machine = Machine::new(args);
    info!("Machine created");

    machine.register_tokens(tokens());
    info!("Tokens registered");

    match machine.preprocess() {
        Ok(_) => {}
        Err(e) => {
            error!("Error during preprocessing: {}", e);
            std::process::exit(1);
        }
    };
    info!("Preprocessing finished");

    info!("Starting lexing");
    match machine.lex() {
        Ok(_) => {}
        Err(e) => {
            error!("Error during lexing: {}", e);
            std::process::exit(1);
        }
    };
    info!("Lexing finished");

    info!("Starting interpretation");
    match machine.interpret() {
        Ok(_) => {}
        Err(e) => {
            error!("Error during interpretation: {}", e);
            std::process::exit(1);
        }
    };
    info!("Interpretation finished");
}
