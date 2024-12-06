use super::args::Args;
use super::engine::file::File;
use super::engine::machine::Machine;
use super::included_libs::libs;
use super::tokens::tokens;
use anyhow::Result;
use log::{error, info};

use std::sync::Once;

static INIT: Once = Once::new();

fn run_file(file: &str) -> Result<bool> {
    INIT.call_once(|| {
        pretty_env_logger::init();
    });
    info!("Starting interpreter on file: {}", file);

    info!("Loading main file: {}", file);
    let mut main_file = match File::new("main".to_string(), file.to_string()) {
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

    let mut machine = Machine::new(
        Args {
            file: file.to_string(),
            debug_inter: false,
            args: vec![],
        },
        main_file,
    );
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

    let stack = machine.stack().elements();
    if stack.len() < 3 {
        error!("Not enough values on stack");
        std::process::exit(1);
    }
    let expected_value = stack.get(0).unwrap().to_string();
    let expected_amount = stack.get(1).unwrap();
    let other_values = stack.get(2..).unwrap().to_vec();
    info!("Stack: {:?}", stack);
    if !expected_amount.is_none() {
        if expected_amount.is_number() && expected_amount.as_int()? != other_values.len() as i32 {
            error!(
                "Expected amount of values on stack does not match: {} != {}",
                expected_amount.as_int()?,
                other_values.len()
            );
            return Ok(false);
        }
    }
    for value in other_values {
        info!("Value on stack: {}", value.to_string());
        if value.to_string() != expected_value {
            error!(
                "Values on stack do not match: {} != {}",
                value.to_string(),
                expected_value
            );
            return Ok(false);
        }
    }

    return Ok(true);
}

#[test]
fn math() {
    let res = match run_file("./tests/math.seal") {
        Ok(res) => res,
        Err(e) => {
            error!("Error running math test: {}", e);
            std::process::exit(1);
        }
    };
    assert_eq!(res, true);
}

#[test]
fn logic() {
    let res = match run_file("./tests/logic.seal") {
        Ok(res) => res,
        Err(e) => {
            error!("Error running logic test: {}", e);
            std::process::exit(1);
        }
    };
    assert_eq!(res, true);
}

#[test]
fn while_loop() {
    let res = match run_file("./tests/while.seal") {
        Ok(res) => res,
        Err(e) => {
            error!("Error running while test: {}", e);
            std::process::exit(1);
        }
    };
    assert_eq!(res, true);
}

#[test]
fn if_statement() {
    let res = match run_file("./tests/if.seal") {
        Ok(res) => res,
        Err(e) => {
            error!("Error running if test: {}", e);
            std::process::exit(1);
        }
    };
    assert_eq!(res, true);
}

#[test]
fn proc() {
    let res = match run_file("./tests/proc.seal") {
        Ok(res) => res,
        Err(e) => {
            error!("Error running proc test: {}", e);
            std::process::exit(1);
        }
    };
    assert_eq!(res, true);
}
