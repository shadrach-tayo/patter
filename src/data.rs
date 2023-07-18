use std::fs::File;
use std::io;

use super::{utils};

// todo: Implement first Ipfs provider to uploading files to ipfs and return cid and etc
pub trait StorageProvider {
    fn name(&self) -> String;
    fn init(&self) -> bool;
    fn api_url(&self) -> String;
    fn pin_file(&mut self, file: File) -> Result<(), io::Error>;
    fn pin_json(&mut self, file: File) -> Result<(), io::Error>;
    fn pin_directory(&mut self, file: File) -> Result<(), io::Error>;
    fn unpin(&mut self, file: File) -> Result<(), io::Error>;
}


#[derive(Debug)]
pub struct PinataProvider {
    pub name: String,
    pub api_url: String,
    pub api_key: String,
    pub secret_api_key: String,
}

impl PinataProvider {
    pub fn new() -> PinataProvider {
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
    fn pin_file(&mut self, file: File) -> Result<(), io::Error> {
        println!("Pinning file to Pinata");
        Ok(())
    }

    #[allow(unused_variables)]
    fn pin_json(&mut self, file: File) -> Result<(), io::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn pin_directory(&mut self, file: File) -> Result<(), io::Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn unpin(&mut self, file: File) -> Result<(), io::Error> {
        todo!()
    }
}

use std::{thread};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::api::data::PinnedObject;
use crate::errors::ApiError;

pub struct PinByFile {
    pub file_path: PathBuf,
    pub providers: Vec<Box<dyn StorageProvider + Send + Sync>>
}

pub struct PatterApi {}

impl PatterApi {
    pub fn new() -> Self {
        PatterApi {}
    }
    pub async fn pin_file(&self, pin_data: PinByFile) -> Result<PinnedObject, ApiError> {
        println!("File path {}, providers: {}", pin_data.file_path.to_str().unwrap_or(""), pin_data.providers.len());

        // todo: read file from disk and share in thread
        let mut handles = vec![];
        for provider in pin_data.providers {
            let provider = Arc::new(provider);
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
        };

        Ok(PinnedObject { ipfs_hash: "".to_string(), pin_size: 5583924, timestamp: "5834773747".to_string() })
    }
}