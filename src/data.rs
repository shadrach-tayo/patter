use std::fs;

// todo: Implement first Ipfs provider to uploading files to ipfs and return cid and etc
pub trait StorageProvider {
    fn name(&self) -> String;
    fn init(&self) -> bool;
    fn api_url(&self) -> String;
    fn pin_file(&mut self, file: &Form) -> Result<PinnedObject, ApiError>;
    fn pin_json(&mut self, file: &Form) -> Result<(), ApiError>;
    fn pin_directory(&mut self, file: &Form) -> Result<(), ApiError>;
    fn unpin(&mut self, file: &Form) -> Result<(), ApiError>;
}

use std::{thread};
use std::path::{Path};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest::multipart::{Form, Part};
use walkdir::WalkDir;

use crate::api::data::PinnedObject;
use crate::errors::ApiError;

pub type SafeStorage = Box<dyn StorageProvider + Send + Sync>;

pub struct PinByFile {
    pub(crate) files: Vec<String>,
    pub(crate) providers: Vec<SafeStorage>
}

impl PinByFile {
    pub fn new<S: Into<String>>(path: S, providers: Vec<SafeStorage>) -> Self {
        PinByFile { files: vec![path.into()], providers }
    }
}

pub struct PatterApi {}

impl PatterApi {
    pub fn new() -> Self {
        PatterApi {}
    }
    pub async fn pin_file(&self, pin_data: PinByFile) -> Result<Vec<PinnedObject>, ApiError> {
        let mut form = Form::new();
        println!("File path {:?}, providers: {}", pin_data.files, pin_data.providers.len());

        for file_data in pin_data.files {
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

                    // let mut mut_form = form.lock().unwrap();
                    // *mut_form = *mut_form.part("file", part);
                    form = form.part("file", part);
                }

            } else {
                let file_name = base_path.file_name().unwrap().to_str().unwrap();
                let part = Part::bytes(fs::read(base_path)?);

                // let mut mut_form = form.lock().unwrap();
                // *mut_form = mut_form.part("file", part.file_name(String::from(file_name)));
                form = form.part("file", part.file_name(String::from(file_name)));
            }
        }

        let mut handles = vec![];
        let results: Arc<Mutex<Vec<PinnedObject>>> = Arc::new(Mutex::new(vec![]));
        let shared_form = Arc::new(form);

        for provider in pin_data.providers {
            let results:  Arc<Mutex<Vec<PinnedObject>>>  = Arc::clone(&results);
            println!("Started Pinning file to provider {}......{:?}", provider.name(), &shared_form);
            let provider = Arc::new(Mutex::new(provider));
            // let p = provider.clone();
            let shared_form = Arc::clone(&shared_form);
            let handle = thread::spawn(move || {
                let mut provider = provider.lock().unwrap();
                thread::sleep(Duration::from_millis(1000));
                println!("Pinning file to provider {}", provider.name());
                let result = provider.pin_file(&shared_form);
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
            handle.join().unwrap();
        };

        let getter = results.lock().unwrap().to_vec();
        Ok(getter)
    }
}