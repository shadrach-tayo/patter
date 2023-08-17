use std::fs;
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, Response};
use reqwest::header::{HeaderMap};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use crate::api::data::{JobStatus, PinByFile, PinByHash, PinByHashResult, PinByJson, PinnedObject, PinnedResult, UnPin};
use crate::data::StorageProvider;
use crate::errors::{Error, ApiError};
use crate::utils::transform_file_to_form;

#[derive(Debug, Deserialize)]
pub(crate) struct Web3StorageApiError {
    name: String,
    message: String,
}

impl Web3StorageApiError {
    pub fn message(&self) -> String {
        format!("{}: {}", self.name.clone(), self.message.clone())
    }
}

#[derive(Debug)]
pub struct Web3StorageProvider {
    pub name: String,
    pub api_url: String,
    client: Client
}

impl Web3StorageProvider {
    pub fn new() -> Result<Web3StorageProvider, Error> {
        let api_token = std::env::var("WEB3STORAGE_API_TOKEN").expect("PINATA_API_KEY env required to run test");

        let mut  default_headers = HeaderMap::new();
        default_headers.insert("Authorization", format!("Bearer {}", api_token).parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(Web3StorageProvider {
            name: "Web3Storage Provider".to_string(),
            api_url: "https://api.web3.storage".to_string(),
            client
        })
    }

    async fn parse_result<R>(&self, response: Response) -> Result<R, ApiError>
        where R: DeserializeOwned
    {
        if response.status().is_success() {
            let json_value = response.json::<R>().await?;
            Ok(json_value)
        } else {
            let error = response.json::<Web3StorageApiError>().await?;
            println!("Error {:?}", error);
            Err(ApiError::GenericError(error.message()))
        }
    }
}

#[async_trait]
impl StorageProvider for Web3StorageProvider {
    fn name(&self) -> String {
        "Web3Storage Provider".to_string()
    }

    fn init(&self) -> bool {
        todo!()
    }
    fn api_url(&self) -> String {
        "https://api.web3.storage".to_string()
    }

    async fn pin_file(&self, pin_data: PinByFile) -> Result<PinnedObject, ApiError> {
        let form = transform_file_to_form(&pin_data)?;

        let response = self.client.post(format!("{}{}", &self.api_url, "/upload"))
            .multipart(form)
            .send()
            .await?;

        let res = self.parse_result::<PinnedResult>(response).await?;
        println!("[Web3StorageProvider::PinFile] {:?}", res);
        Ok(PinnedObject { ipfs_hash: res.cid, timestamp: "".to_string(), pin_size: 0 })
    }

    async fn pin_json(&self, pin_data: PinByJson) -> Result<PinnedObject, ApiError> {
        let file = fs::read_to_string(pin_data.file)?;
        let data:serde_json::Value = serde_json::from_str(file.as_str()).expect("Could not parse json file");
        let response = self.client.post(format!("{}{}", &self.api_url, "/upload"))
            .json(&data)
            .send()
            .await?;

        let res = self.parse_result::<PinnedResult>(response).await?;
        println!("[Web3StorageProvider::PinJson] {:?}", res);
        Ok(PinnedObject { ipfs_hash: res.cid, timestamp: "".to_string(), pin_size: 0 })
    }

    async fn pin_by_hash(&self, pin_data: PinByHash) -> Result<PinByHashResult, ApiError> {
        Err(ApiError::GenericError(format!("Pin by Hash not Implemented for Web3Storage, hash: {}", &pin_data.hash_to_pin)))
    }

    async fn pin_directory(&self) -> Result<(), ApiError> {
        todo!()
    }

    async fn unpin(&self, data: UnPin) -> Result<(), ApiError> {
        Err(ApiError::GenericError(format!("UnPin Cid feature not Implemented for Web3Storage, CID: {}", &data.cid)))
    }
}