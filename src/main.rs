use std::{io, process};
use clap::Parser;
use patter::Args;
use dotenv::dotenv;
//  cargo run -- --action pin_file --file-path ./cargo.toml --provider pinata

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv().ok();
    let arg = Args::parse();
    dbg!(&arg);
    if let Err(e) = patter::run(arg).await {
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
