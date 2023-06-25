use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::{fs, io};

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
/// let arg = patter::Args { file_path: PathBuf::from("./cargo.toml".to_string()), action: "pin_file".to_string(), provider: Some("pinata".to_string())};
/// let result = patter::run(arg);
/// assert_eq!(result, ());
/// ```
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let providers: Vec<Box<dyn StorageProvider>> = if let Some(provider) = args.provider {
        match provider.as_str() {
            "pinata" => {
                vec![Box::new(PinataProvider::new())]
            }
            _ => {
                panic!("Unsupported provider");
            }
        }
    } else {
        vec![Box::new(PinataProvider::new())]
    };

    let names = providers.iter().map(|p| p.name()).collect::<Vec<String>>();
    println!("Uploading to the providers: {:?}", names);

    match args.action.as_str() {
        "pin_file" => {
            println!("pin files");
            // todo: read file from disk and share in thread
            // todo: map providers to create a thread of |provider| => provider.pin_file(&file)
        }
        _ => {
            panic!("Specify what you want to do.\n \
            use `--action pin_file` to pin a file\n
            ")
        }
    };

    Ok(())
}

// todo: Define generic trait for storage providers
// todo: Implement first Ipfs provider to uploading files to ipfs and return cid and etc

trait StorageProvider {
    fn name(&self) -> String;
    fn init(&mut self) -> bool;
    fn api_url(&self) -> String;
    fn pin_file(&mut self, file: File) -> Result<(), std::io::Error>;
    fn pin_json(&mut self, file: File) -> Result<(), std::io::Error>;
    fn pin_directory(&mut self, file: File) -> Result<(), std::io::Error>;
    fn unpin(&mut self, file: File) -> Result<(), std::io::Error>;
}

#[derive(Debug)]
pub struct PinataProvider {
    pub name: String,
    pub api_url: String,
    pub jwt: String,
}

impl PinataProvider {
    fn new() -> PinataProvider {
        PinataProvider {
            name: "Pinata Provider".to_string(),
            api_url: "https://api.pinata.cloud/".to_string(),
            jwt: "".to_string(),
        }
    }
}

impl StorageProvider for PinataProvider {
    fn name(&self) -> String {
        "Pinata Provider".to_string()
    }

    fn init(&mut self) -> bool {
        println!("Initializing Pinata");
        let mut token: String = "".to_string();
        println!("Enter your Pinata jwt key");

        // todo: make this optional to skip uploading to pinata
        io::stdin()
            .read_line(&mut token)
            .expect("Please enter your jwt api key");
        self.jwt = token;
        true
    }

    fn api_url(&self) -> String {
        "https://api.pinata.cloud/".to_string()
    }

    #[allow(unused_variables)]
    fn pin_file(&mut self, file: File) -> Result<(), std::io::Error> {
        println!("Pinning file to Pinata");
        Ok(())
    }

    #[allow(unused_variables)]
    fn pin_json(&mut self, file: File) -> Result<(), std::io::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn pin_directory(&mut self, file: File) -> Result<(), std::io::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn unpin(&mut self, file: File) -> Result<(), std::io::Error> {
        todo!()
    }
}
