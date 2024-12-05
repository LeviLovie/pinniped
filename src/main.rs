pub mod args;
pub mod engine;
pub mod tokens;
pub mod included_libs;

use log::{error, info};

use crate::engine::machine::Machine;
use crate::{tokens::tokens, included_libs::libs};

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

    let mut machine = Machine::new(args);
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
