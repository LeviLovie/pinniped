use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "./main.seal")]
    pub file: String,

    #[arg(short, long, default_value = "false")]
    pub debug_inter: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub args: Vec<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
