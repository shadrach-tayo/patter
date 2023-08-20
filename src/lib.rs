use std::fmt::Debug;
use clap::{arg, Parser};
use data::{StorageProvider,  PatterApi, SafeStorage};
use api::data::PinnedObject;
use errors::*;
use providers::pinata::{PinataProvider};
use crate::api::data::PinByHashResult;
use crate::data::{PinFileData, PinHashData, PinJsonData};
use crate::providers::web3Storage::Web3StorageProvider;

mod utils;
mod api;
mod data;
mod errors;
mod providers;

#[allow(unused_variables)]

/// Cli app to upload files to ipfs storage provider
#[derive(Parser, Debug)]
#[clap(author="Patter", about="A rust library for pinning data to ipfs")]
pub struct Args {
    /// ipfs storage provider api key
    #[arg(short, long)]
    pub action: String,

    #[arg(short, long)]
    pub provider: Option<String>,

    /// Path to file to be uploaded
    #[arg(short, long)]
    pub file_path: Option<String>,/// Path to file to be uploaded

    #[arg(long)]
    pub hash: Option<String>,
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
/// # tokio_test::block_on(async {
///     let arg = patter::Args { hash: None, file_path: Some("./cargo.toml".to_string()), action: "pin_file".to_string(), provider: Some("pinata".to_string())};
///     let result = patter::run(arg).await.unwrap();
///     assert_eq!(result, ());
/// # })
/// ```
pub async fn run(args: Args) -> Result<(), &'static str> {

    let providers: Vec<Box<dyn StorageProvider + Send + Sync>> = if let Some(provider) = args.provider {
        match provider.as_str() {
            "pinata" => {
                vec![Box::new(PinataProvider::new(None, None).unwrap()) as SafeStorage]
            }
            "web3" => {
                vec![Box::new(Web3StorageProvider::new(None).unwrap()) as SafeStorage]
            }
            _ => {
                panic!("Unsupported provider");
            }
        }
    } else {
        vec![Box::new(PinataProvider::new(None, None).unwrap()) as SafeStorage, Box::new(Web3StorageProvider::new(None).unwrap()) as SafeStorage]
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

            let result: Result<Vec<PinnedObject>, ApiError> = patter_api.pin_file(PinFileData { files: vec![args.file_path.unwrap()], providers }).await;
            println!("[patter_api.pin_file]:: {:?}", result.unwrap());
        }
        "pin_json" => {
            println!("pin json");
            let patter_api = PatterApi::new();

            let result: Result<Vec<PinnedObject>, ApiError> = patter_api.pin_json(PinJsonData { file: args.file_path.unwrap(), providers }).await;
            println!("[patter_api.pin_json]:: {:?}", result.unwrap());
        }
        "pin_hash" => {
            println!("....pin hash....");
            let patter_api = PatterApi::new();

            let result: Result<Vec<PinByHashResult>, ApiError> = patter_api.pin_by_hash(PinHashData { hash: args.hash.unwrap(), providers }).await;
            println!("[patter_api.pin_hash]:: {:?}", result.unwrap());
        }
        "unpin" => {
            println!("....removing cid....");
            let patter_api = PatterApi::new();

            let result: Result<(), ApiError> = patter_api.unpin(PinHashData { hash: args.hash.unwrap(), providers }).await;
            println!("[patter_api.unpin]:: {:?}", result.unwrap());
        }
        _ => {
            panic!("Specify what you want to do.\n \
            use `--action pin_file` to pin a file\n
            ")
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests;
