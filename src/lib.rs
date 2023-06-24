use std::error::Error;
use std::fs;
use clap::Parser;

/// Cli app to upload files to ipfs storage provider
#[derive(Parser, Debug)]
pub struct Args {
    /// ipfs storage provider api key
    #[arg(short, long)]
    pub api_key: String,

    /// Path to file to be uploaded
    #[arg(short, long)]
    pub file_path: std::path::PathBuf,
}

/// Takes an arg of type Args and runs the app using the
/// the config
///
/// # Example
/// ```
/// use std::env;
/// use std::path::PathBuf;
/// let cwd = env::current_dir().unwrap();
/// let path = String::from(cwd.to_string_lossy());
/// let arg = patter::Args { file_path: PathBuf::from("./cargo.toml".to_string()), api_key: "api-key".to_string()};
/// let result = patter::run(arg);
/// ```
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;
    dbg!(contents);
    Ok(())
}