use async_trait::async_trait;
use std::{thread};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::api::data::{PinByFile, PinnedObject};
use crate::errors::ApiError;

// todo: Implement first Ipfs provider to uploading files to ipfs and return cid and etc
#[async_trait]
pub trait StorageProvider {
    fn name(&self) -> String;
    fn init(&self) -> bool;
    fn api_url(&self) -> String;
    async fn pin_file(&self,  pin_data: PinByFile) -> Result<PinnedObject, ApiError>;
    async fn pin_json(&self,) -> Result<(), ApiError>;
    async fn pin_directory(&self) -> Result<(), ApiError>;
    async fn unpin(&self) -> Result<(), ApiError>;
}

pub type SafeStorage = Box<dyn StorageProvider + Send + Sync>;

pub struct PinFileData {
    pub(crate) files: Vec<String>,
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
            println!("Pinning file to provider {}", provider.name());
            let handle = thread::spawn(move || async move {
                thread::sleep(Duration::from_millis(1000));
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
}

// async fn run(provider: Arc<SafeStorage>,files: Arc<Vec<String>>) -> Result<PinnedObject, ApiError> {
//     // let mut provider = provider.lock().unwrap();
//     thread::sleep(Duration::from_millis(1000));
//     println!("Pinning file to provider {}", provider.name());
//     // let data = PinByFile { files: files.to_vec() };
//     let result = provider.pin_file(PinByFile { files: files.to_vec() }).await;
//     if let Ok(pinned_object) =  result {
//         println!("Pinned Result {:?} to provider {}", pinned_object, provider.name());
//         // let mut r = results.lock().unwrap();
//         // r.push(pinned_object);
//         // r
//         Ok(pinned_object)
//     } else {
//         println!("Error Pinning file to provider {}", provider.name());
//         println!("Error {:?}", result);
//         result
//     }
// }