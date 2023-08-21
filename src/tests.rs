use log::debug;
use crate::api::data::{PinByHash};
use crate::data::{PatterApi, PinFileData, PinJsonData, SafeStorage, StorageProvider};
use crate::providers::pinata::PinataProvider;
use crate::providers::web3Storage::Web3StorageProvider;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// #[tokio::test]
// async fn run_patter() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("patter")?;
//     cmd.arg("-p patter --lib").arg("--action pin_file --file_path ./test.json --provider web3");
//     // .arg("pin_file").arg("file_path").arg("./test.json").cmd.arg("provider").arg("web3");
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("error"));
//     println!("[RUN PATTER]");
//     Ok(())
// }

fn get_pinata_provider() -> PinataProvider {
    let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY env required to run test");
    let secret_api_key = std::env::var("PINATA_SECRET_API_KEY").expect("PINATA_SECRET_API_KEY env required to run test");
    println!("{} {}", &api_key, &secret_api_key);
    PinataProvider::new(Some(api_key), Some(secret_api_key)).unwrap()
}

fn get_web3_provider() -> Web3StorageProvider {
    let token = std::env::var("WEB3STORAGE_API_TOKEN").expect("WEB3STORAGE_API_TOKEN env required to run test");
    Web3StorageProvider::new(Some(token)).unwrap()
}


#[tokio::test]
async fn test_pin_by_hash() {
    let result = get_pinata_provider().pin_by_hash(PinByHash { hash_to_pin: "QmVk4Fwc2zy4FfDyVAjPQ3qoGM8Zi4qu8fNcPJ1kvicJyR".to_string() }).await;
    match result {
        Ok(data) => {
            debug!("{:?}", data);
            assert_eq!(data.ipfs_hash, "QmVk4Fwc2zy4FfDyVAjPQ3qoGM8Zi4qu8fNcPJ1kvicJyR".to_string())
        }
        Err(e) => assert!(false, "{}", e),
    }
}

#[tokio::test]
async fn test_pin_file() {
    let patter_api = PatterApi::new();
    let providers = vec![Box::new(get_pinata_provider()) as SafeStorage, Box::new(get_web3_provider()) as SafeStorage];
    let result = patter_api.pin_file(PinFileData { files: vec!["./LICENSE".to_string()], providers }).await;

    match result {
        Ok(pinned_data) => {
            debug!("{:?}", pinned_data);
            let pinata_result = &pinned_data[0];
            assert_eq!(pinata_result.ipfs_hash, "QmT5zMbasLqSDw4MnbZPmgWnnhGpf7BNs32keMYeL7vrhP".to_string());
            let web3_result = &pinned_data[1];
            assert_eq!(web3_result.ipfs_hash, "bafkreigoyk5mbk347djnhxixlbzxj5e5xssf2v5iwzttr26bhuz7ymo6ie".to_string());
        }
        Err(e) => assert!(false, "{}", e),
    }
}

#[tokio::test]
async fn test_pin_json() {
    let patter_api = PatterApi::new();
    let providers = vec![Box::new(get_pinata_provider()) as SafeStorage, Box::new(get_web3_provider()) as SafeStorage];
    let result = patter_api.pin_json(PinJsonData { file: "./test.json".to_string(), providers }).await;

    match result {
        Ok(json_result) => {
            debug!("{:?}", json_result);
            let pinata_result = &json_result[0];
            assert_eq!(pinata_result.ipfs_hash, "QmQWsc8DiUex5hjyGYiTAym1373LUmt1BFVF2E1yZYRc8Z".to_string());
            let web3_result = &json_result[1];
            assert_eq!(web3_result.ipfs_hash, "bafkreiasgxafnvedpdjj4djru2edya7tavjoymh4uxvgpvbpxrl2qxoz64".to_string());
        }
        Err(e) => assert!(false, "{}", e),
    }
}
