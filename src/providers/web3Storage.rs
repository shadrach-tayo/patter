use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, Response};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use crate::api::data::{PinByFile, PinByHash, PinByHashResult, PinByJson, PinnedObject};
use crate::data::StorageProvider;
use crate::errors::{Error, ApiError};

#[derive(Debug, Deserialize)]
pub(crate) struct Web3StorageApiError {
    error: String
}

impl Web3StorageApiError {
    pub fn message(&self) -> String {
        self.error.clone()
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
        let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY env required to run test");
        let secret_api_key = std::env::var("PINATA_SECRET_API_KEY").expect("SECRET_API_KEY env required to run test");

        let mut  default_headers = HeaderMap::new();
        default_headers.insert("pinata_api_key", api_key.parse().unwrap());
        default_headers.insert("pinata_secret_api_key", secret_api_key.parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(Web3StorageProvider {
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
            let error = response.json::<Web3StorageApiError>().await?;
            println!("Error {:?}", error);
            Err(ApiError::GenericError(error.message()))
        }
    }
}

#[async_trait]
impl StorageProvider for Web3StorageProvider {
    fn name(&self) -> String {
        todo!()
    }

    fn init(&self) -> bool {
        todo!()
    }
    fn api_url(&self) -> String {
        todo!()
    }

    async fn pin_by_hash(&self, pin_data: PinByHash) -> Result<PinByHashResult, ApiError> {
        todo!()
    }

    async fn pin_directory(&self) -> Result<(), ApiError> {
        todo!()
    }

    async fn pin_file(&self, pin_data: PinByFile) -> Result<PinnedObject, ApiError> {
        todo!()
    }

    async fn pin_json(&self, pin_data: PinByJson) -> Result<PinnedObject, ApiError> {
        todo!()
    }

    async fn unpin(&self) -> Result<(), ApiError> {
        todo!()
    }
}