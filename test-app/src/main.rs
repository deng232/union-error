mod error;
mod file1;
mod file2;

use crate::error::Result;

fn run() -> Result<()> {
    let n1 = file1::parse_number("123")?;
    println!("parsed from file1: {n1}");

    let n2 = file2::read_and_parse("number.txt")?;
    println!("parsed from file2: {n2}");

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");

        let mut source = std::error::Error::source(&err);
        while let Some(err) = source {
            eprintln!("caused by: {err}");
            source = err.source();
        }

        std::process::exit(1);
    }
}
