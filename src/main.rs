use std::{io, process};
use clap::Parser;
use patter::Args;
use dotenv::dotenv;
//  cargo run -- --action pin_file --file-path ./cargo.toml --provider pinata

fn main() -> Result<(), io::Error> {
    dotenv().ok();
    // let dotenv_path = dotenv().expect("failed to find .env file");
    // println!("dotenvpath {}", dotenv_path);
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

    #[test]
    fn all_pass() {
        assert_eq!(true, true);
    }
}
