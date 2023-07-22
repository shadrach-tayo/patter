use std::io;
use reqwest::{Client, ClientBuilder};
use reqwest::header::HeaderMap;
use reqwest::multipart::Form;
use crate::api::data::PinnedObject;
use crate::data::StorageProvider;
use crate::errors::{ApiError, Error};
use crate::utils;

#[derive(Debug)]
pub struct PinataProvider {
    pub name: String,
    pub api_url: String,
    client: Client
}

impl PinataProvider {
    pub fn new() -> Result<PinataProvider, Error> {
        let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY env required to run test");
        let secret_api_key = std::env::var("SECRET_API_KEY").expect("SECRET_API_KEY env required to run test");

        let mut  default_headers = HeaderMap::new();
        default_headers.insert("pinata_api_key", api_key.parse().unwrap());
        default_headers.insert("pinata_secret_api_key", secret_api_key.parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(PinataProvider {
            name: "Pinata Provider".to_string(),
            api_url: "https://api.pinata.cloud/".to_string(),
            client
        })
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
    fn pin_file(&mut self, file: &Form) -> Result<PinnedObject, ApiError> {
        println!("Pinning file to Pinata");
        Ok(PinnedObject { ipfs_hash: "".to_string(), pin_size: 5583924, timestamp: "9864773747".to_string() })
    }

    #[allow(unused_variables)]
    fn pin_json(&mut self, file: &Form) -> Result<(), ApiError> {
        todo!()
    }

    #[allow(unused_variables)]
    fn pin_directory(&mut self, file: &Form) -> Result<(), ApiError> {
        todo!()
    }

    #[allow(unused_variables)]
    fn unpin(&mut self, file: &Form) -> Result<(), ApiError> {
        todo!()
    }
}
