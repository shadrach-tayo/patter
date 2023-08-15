use std::fs;
use std::path::Path;
use reqwest::multipart::{Form, Part};
use walkdir::WalkDir;
use crate::api::data::PinByFile;
use crate::errors::ApiError;

pub fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s.to_owned()
}

pub fn transform_file_to_form(pin_data: &PinByFile) -> Result<Form, ApiError> {
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
    };

    Ok(form)
}