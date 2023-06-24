use std::{io, path};
use clap::Parser;

/// Cli app to upload files to ipfs storage provider
#[derive(Parser, Debug)]
struct Args {
    /// ipfs storage provider api key
    #[arg(short, long)]
    api_key: String,

    /// Path to file to be uploaded
    #[arg(short, long)]
    file_path: path::PathBuf,
}

fn main() -> Result<(), io::Error> {
    let arg = Args::parse();
    dbg!(&arg);
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
