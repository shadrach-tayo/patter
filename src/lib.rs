use clap::Parser;
use std::fs::File;
use std::{io, thread};
use std::sync::Arc;
use std::time::Duration;

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
pub fn run(args: Args) -> Result<(), &'static str> {
    let providers: Vec<Arc<Box<dyn StorageProvider + Send + Sync>>> = if let Some(provider) = args.provider {
        match provider.as_str() {
            "pinata" => {
                vec![Arc::new(Box::new(PinataProvider::new()) as Box<dyn StorageProvider + Send + Sync>)]
            }
            _ => {
                panic!("Unsupported provider");
            }
        }
    } else {
        vec![Arc::new(Box::new(PinataProvider::new()) as Box<dyn StorageProvider + Send + Sync>)]
    };

    let names = providers.iter().map(|p| p.name()).collect::<Vec<String>>();
    println!("Uploading to the providers: {:?}", names);

    if providers.len() == 0 {
        return Err("No Valid provider");
    }
    match args.action.as_str() {
        "pin_file" => {
            println!("pin files");
            // todo: read file from disk and share in thread
            // todo: map providers to create a thread of |provider| => provider.pin_file(&file)
            let mut handles = vec![];
            for provider in providers {
                let p = provider.clone();
                let handle = thread::spawn(move || {
                    println!("Started Pinning file to provider {}......", p.name());
                    thread::sleep(Duration::from_millis(5000));
                    println!("Pinning file to provider {}", p.name());
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }

        }
        _ => {
            panic!("Specify what you want to do.\n \
            use `--action pin_file` to pin a file\n
            ")
        }
    };

    Ok(())
}

// todo: Implement first Ipfs provider to uploading files to ipfs and return cid and etc
trait StorageProvider {
    fn name(&self) -> String;
    fn init(&self) -> bool;
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
    pub api_key: String,
    pub secret_api_key: String,
}

impl PinataProvider {
    fn new() -> PinataProvider {
        let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY env required to run test");
        let secret_api_key = std::env::var("SECRET_API_KEY").expect("SECRET_API_KEY env required to run test");

        PinataProvider {
            name: "Pinata Provider".to_string(),
            api_url: "https://api.pinata.cloud/".to_string(),
            api_key,
            secret_api_key,
        }
    }
}

mod utils;
impl StorageProvider for PinataProvider {
    fn name(&self) -> String {
        "Pinata Provider".to_string()
    }

    fn init(&self) -> bool {
        println!("Initializing Pinata");
        let mut token: String = "".to_string();
        println!("Enter your Pinata jwt key");

        // todo: make this optional to skip uploading to pinata
        io::stdin()
            .read_line(&mut token)
            .expect("Please enter your jwt api key");

        // todo: read details from env
        let token = utils::trim_newline(&mut token);
        if token == String::from("") {
            return false;
        };
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
