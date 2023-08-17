use std::{fs, io};
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, Response};
use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use serde::de::DeserializeOwned;
use crate::api::data::{PinnedObject, PinByFile, PinByJson, PinByHash, PinByHashResult, UnPin};
use crate::data::StorageProvider;
use crate::errors::{ApiError, Error};
use crate::utils;
use serde::Deserialize;
use crate::utils::transform_file_to_form;

#[derive(Deserialize, Debug)]
pub(crate) struct PinataApiError {
    error: String
}

impl PinataApiError {
    pub fn message(&self) -> String {
        self.error.clone()
    }
}

#[derive(Debug)]
pub struct PinataProvider {
    pub name: String,
    pub api_url: String,
    client: Client
}

impl PinataProvider {
    pub fn new() -> Result<PinataProvider, Error> {
        let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY env required to run test");
        let secret_api_key = std::env::var("PINATA_SECRET_API_KEY").expect("SECRET_API_KEY env required to run test");

        let mut  default_headers = HeaderMap::new();
        default_headers.insert("pinata_api_key", api_key.parse().unwrap());
        default_headers.insert("pinata_secret_api_key", secret_api_key.parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(PinataProvider {
            name: "Pinata Provider".to_string(),
            api_url: "https://api.pinata.cloud".to_string(),
            client
        })
    }

    async fn parse_result<R>(&self, response: Response) -> Result<R, ApiError>
      where R: DeserializeOwned
    {
        if response.status().is_success() {
            let result = response.json::<R>().await?;
            Ok(result)
        } else {
            let error = response.json::<PinataApiError>().await?;
            println!("Error {:?}", error);
            Err(ApiError::GenericError(error.message()))
        }
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
        "https://api.pinata.cloud".to_string()
    }

    async fn pin_file(&self, pin_data: PinByFile) -> Result<PinnedObject, ApiError> {
        let form = transform_file_to_form(&pin_data)?; // Form::new();

        let response = self.client.post(format!("{}{}", &self.api_url, "/pinning/pinFileToIPFS"))
            .multipart(form)
            .send()
            .await?;

        self.parse_result(response).await
        // Ok(PinnedObject { ipfs_hash: "".to_string(), pin_size: 5583924, timestamp: "9864773747".to_string() })
    }

    #[allow(unused_variables)]
    async fn pin_json(&self, pin_data: PinByJson) -> Result<PinnedObject, ApiError> {
        let file = fs::read_to_string(pin_data.file)?;
        let data:serde_json::Value = serde_json::from_str(file.as_str()).expect("Could not parse json file");
        let response = self.client.post(format!("{}{}", &self.api_url, "/pinning/pinJSONToIPFS"))
            .json(&data)
            .send()
            .await?;

        self.parse_result(response).await
    }

 #[allow(unused_variables)]
    async fn pin_by_hash(&self, pin_data: PinByHash) -> Result<PinByHashResult, ApiError> {
        let response = self.client.post(format!("{}{}", &self.api_url, "/pinning/pinByHash"))
            .json(&pin_data)
            .send()
            .await?;

        self.parse_result(response).await
    }

    #[allow(unused_variables)]
    async fn pin_directory(&self) -> Result<(), ApiError> {
        todo!()
    }

    #[allow(unused_variables)]
    async fn unpin(&self, param: UnPin) -> Result<(), ApiError> {
        let response = self.client.delete(format!("{}{}{}", &self.api_url, "/pinning/unpin/", &param.cid))
            .send()
            .await?;

        let is_success = response.status().is_success();
        println!("UnPin result {:?}", is_success);
        if is_success {
             Ok(())
        } else {
            Err(ApiError::GenericError("Error unpinning cid from Pinata".to_string()))
        }
    }
}
