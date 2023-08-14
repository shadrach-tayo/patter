use async_trait::async_trait;
use std::{thread};
use std::future::Future;
use std::sync::{Arc, Mutex};

use crate::api::data::{PinByFile, PinByHash, PinByHashResult, PinByJson, PinnedObject};
use crate::errors::ApiError;

// todo: Implement first Ipfs provider to uploading files to ipfs and return cid and etc
#[async_trait]
pub trait StorageProvider {
    fn name(&self) -> String;
    fn init(&self) -> bool;
    fn api_url(&self) -> String;
    async fn pin_file(&self,  pin_data: PinByFile) -> Result<PinnedObject, ApiError>;
    async fn pin_json(&self, pin_data: PinByJson) -> Result<PinnedObject, ApiError>;
    async fn pin_by_hash(&self, pin_data: PinByHash) -> Result<PinByHashResult, ApiError>;
    async fn pin_directory(&self) -> Result<(), ApiError>;
    async fn unpin(&self) -> Result<(), ApiError>;
}

pub type SafeStorage = Box<dyn StorageProvider + Send + Sync>;

pub struct PinFileData {
    pub(crate) files: Vec<String>,
    pub(crate) providers: Vec<SafeStorage>
}

pub struct PinJsonData {
    pub(crate) file: String,
    pub(crate) providers: Vec<SafeStorage>
}

pub struct  PinHashData {
    pub(crate) hash: String,
    pub(crate) providers: Vec<SafeStorage>
}

impl PinFileData {
    #[allow(unused)]
    pub fn new<S: Into<String>>(path: S, providers: Vec<SafeStorage>) -> Self {
        PinFileData { files: vec![path.into()], providers }
    }
}

pub struct PatterApi {}

impl PatterApi {
    pub fn new() -> Self {
        PatterApi {}
    }

    pub async fn pin_file(&self, pin_data: PinFileData) -> Result<Vec<PinnedObject>, ApiError> {
        let mut handles = vec![];

        let results: Arc<Mutex<Vec<PinnedObject>>> = Arc::new(Mutex::new(vec![]));
        let files = Arc::new(pin_data.files);
        for provider in pin_data.providers {
            let results:  Arc<Mutex<Vec<PinnedObject>>>  = Arc::clone(&results);
            let provider = Arc::new(provider);
            let files = Arc::clone(&files);
            println!("Creating async thread for provider {}", provider.name());

            let handle = thread::spawn(move || async move {
                let result = provider.pin_file(PinByFile { files: files.to_vec() }).await;
                if let Ok(pinned_object) =  result {
                    println!("Pinned Result {:?} to provider {}", pinned_object, provider.name());
                    let mut r = results.lock().unwrap();
                    r.push(pinned_object);
                } else {
                    println!("Error Pinning file to provider {}", provider.name());
                    println!("Error {:?}", result);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap().await;
        };

        let getter = results.lock().unwrap().to_vec();
        Ok(getter)
    }

    pub async fn pin_json(&self, pin_data: PinJsonData) -> Result<Vec<PinnedObject>, ApiError> {
        let mut handles = vec![];

        let results: Arc<Mutex<Vec<PinnedObject>>> = Arc::new(Mutex::new(vec![]));
        let file = Arc::new(pin_data.file);
        for provider in pin_data.providers {
            let results:  Arc<Mutex<Vec<PinnedObject>>>  = Arc::clone(&results);
            let provider = Arc::new(provider);
            let file = Arc::clone(&file);
            println!("Creating async thread for provider {}", provider.name());
            let handle = thread::spawn(move || async move {
                let result = provider.pin_json(PinByJson { file: file.to_string() }).await;
                match result {
                    Ok(pinned_json) => {
                        println!("Pinned Result {:?} to provider {}", pinned_json, provider.name());
                        let mut r = results.lock().unwrap();
                        r.push(pinned_json);
                    }
                    Err(..) => {
                        println!("Error Pinning file to provider {}", provider.name());
                        println!("Error {:?}", result);
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap().await;
        };

        let getter = results.lock().unwrap().to_vec();
        Ok(getter)
    }

    pub async fn pin_by_hash(&self, pin_data: PinHashData) -> Result<Vec<PinByHashResult>, ApiError> {
        let mut handles = vec![];

        let results: Arc<Mutex<Vec<PinByHashResult>>> = Arc::new(Mutex::new(vec![]));
        let hash = Arc::new(pin_data.hash);
        for provider in pin_data.providers {
            let results:  Arc<Mutex<Vec<PinByHashResult>>>  = Arc::clone(&results);
            let provider = Arc::new(provider);
            let hash = Arc::clone(&hash);
            println!("Pin hash: {}", &hash);
            let handle = thread::spawn(move || async move {
                let result = provider.pin_by_hash(PinByHash { hash_to_pin: hash.to_string() }).await;
                if let Ok(pinned_hash) =  result {
                    println!("Pinned Result {:?} to provider {}", pinned_hash, provider.name());
                    let mut r = results.lock().unwrap();
                    r.push(pinned_hash);
                } else {
                    println!("Error Pinning hash to {}", provider.name());
                    println!("Error {:?}", result);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap().await;
        };

        let getter = results.lock().unwrap().to_vec();
        Ok(getter)
    }
}
