use clap::{Parser};
use crate::calculator::Calculator;
use clap::error::{Error, ErrorKind};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "help me")]
    divend1: String,

    #[arg(help = "help me")]
    divend2: String,
}

pub fn start() -> Result<(), Error> {
    let args = Args::parse();

    let calculator = Calculator::new_str(args.divend1, args.divend2).map_err(|_| Error::new(ErrorKind::InvalidValue))?;
    let result = calculator.calc();

    println!("{}", result);

    Ok(())
}