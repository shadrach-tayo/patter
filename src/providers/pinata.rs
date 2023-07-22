use std::{fs, io};
use std::path::Path;
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder};
use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use walkdir::WalkDir;
use crate::api::data::{PinnedObject, PinByFile};
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

#[async_trait]
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

    async fn pin_file(&self, pin_data: PinByFile) -> Result<PinnedObject, ApiError> {
        println!("Pinning file to Pinata");
        let mut form = Form::new();
        println!("File path {:?}", pin_data.files);

        for file_data in pin_data.files.iter() {
            let base_path = Path::new(&file_data);

            if base_path.is_dir() {
                // recursively read the directory
                for entry_result in WalkDir::new(base_path) {
                    let entry = entry_result?;
                    let path = entry.path();

                    // not interested in reading directory
                    if path.is_dir() { continue }

                    let path_name = path.strip_prefix(base_path)?;
                    let part_file_name = format!("{}/{}", base_path.file_name().unwrap().to_str().unwrap(), path_name.to_str().unwrap());

                    let part = Part::bytes(fs::read(path)?)
                        .file_name(part_file_name);
                    form = form.part("file", part);
                }

            } else {
                let file_name = base_path.file_name().unwrap().to_str().unwrap();
                let part = Part::bytes(fs::read(base_path)?);
                form = form.part("file", part.file_name(String::from(file_name)));
            }
        }
        println!("Started Pinning file to provider {}......{:?}", &self.name(), &pin_data.files);

        // let response = self.client.post(format!("{}{}", &self.api_url, "/pinning/pinFileToIPFS"))
        //     .multipart(form)
        //     .send()
        //     .await?;

        Ok(PinnedObject { ipfs_hash: "".to_string(), pin_size: 5583924, timestamp: "9864773747".to_string() })

    }

    #[allow(unused_variables)]
    async fn pin_json(&self) -> Result<(), ApiError> {
        todo!()
    }

    #[allow(unused_variables)]
    async fn pin_directory(&self) -> Result<(), ApiError> {
        todo!()
    }

    #[allow(unused_variables)]
    async fn unpin(&self) -> Result<(), ApiError> {
        todo!()
    }
}
