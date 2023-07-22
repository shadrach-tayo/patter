use clap::{arg, Parser};
use data::{StorageProvider,  PatterApi, PinByFile, SafeStorage};
use api::data::PinnedObject;
use errors::*;
use providers::pinata::{PinataProvider};

mod utils;
mod api;
mod data;
mod errors;
mod providers;

#[allow(unused_variables)]

/// Cli app to upload files to ipfs storage provider
#[derive(Parser, Debug)]
pub struct Args {
    /// ipfs storage provider api key
    #[arg(short, long)]
    pub action: String,

    #[arg(short, long)]
    pub provider: Option<String>,

    /// Path to file to be uploaded
    #[arg(short, long)]
    pub file_path: String,
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
/// let arg = patter::Args { file_path: "./cargo.toml".to_string(), action: "pin_file".to_string(), provider: Some("pinata".to_string())};
/// let result = patter::run(arg);
/// assert_eq!(result, ());
/// ```
pub async fn run(args: Args) -> Result<(), &'static str> {

    let providers: Vec<Box<dyn StorageProvider + Send + Sync>> = if let Some(provider) = args.provider {
        match provider.as_str() {
            "pinata" => {
                vec![Box::new(PinataProvider::new().unwrap()) as SafeStorage]
            }
            _ => {
                panic!("Unsupported provider");
            }
        }
    } else {
        vec![Box::new(PinataProvider::new().unwrap()) as SafeStorage]
    };

    let names = providers.iter().map(|p| p.name()).collect::<Vec<String>>();
    println!("Uploading to the providers: {:?}", names);

    if providers.len() == 0 {
        return Err("No Valid provider");
    }
    match args.action.as_str() {
        "pin_file" => {
            println!("pin files");
            let patter_api = PatterApi::new();

            let result: Result<Vec<PinnedObject>, ApiError> = patter_api.pin_file(PinByFile::new(args.file_path, providers ) ).await;
            println!("{:?}", result.unwrap());
        }
        _ => {
            panic!("Specify what you want to do.\n \
            use `--action pin_file` to pin a file\n
            ")
        }
    };
    Ok(())
}

