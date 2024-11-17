use std::{error::Error, fs::read_to_string, path::PathBuf};

use clap::Parser;
use nom::{self, Parser as _};
use splash::{parse::SplashParser, run::Runtime, utils::locate_error};

#[derive(Parser)]
struct Args {
    file: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let args = Args::parse();
    let input = read_to_string(args.file)?;
    let input = format!("{{ {input} }}");

    let mut parser = SplashParser;

    match parser.parse(&input) {
        Ok((_, program)) => {
            if let Err(error) = Runtime::new(program).start() {
                eprintln!("Error: {error}");
            }
        }
        Err(error) => match error {
            nom::Err::Error(error) | nom::Err::Failure(error) => {
                match locate_error(&input, &error) {
                    Some((line, column)) => eprintln!("Error at line {line}, column {column}."),
                    None => eprintln!("Unexpected error: {error:?}"),
                }
            }
            nom::Err::Incomplete(_) => unreachable!(),
        },
    }

    Ok(())
}
