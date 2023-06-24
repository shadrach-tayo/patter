use std::{io, process};
use clap::Parser;
use patter::Args;

fn main() -> Result<(), io::Error> {
    let arg = Args::parse();
    dbg!(&arg);
    if let Err(e) = patter::run(arg) {
        println!("Application error {e}");
        process::exit(1);
    } else {
        println!("Files backed up ✅ ✅  ");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn all_pass() {
        assert_eq!(true, true);
    }
}
