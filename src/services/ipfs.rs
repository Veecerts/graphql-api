use std::io::Read;

use async_graphql::*;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};
use serde::Deserialize;

use crate::config::settings::{EnvVariables, Settings};

pub async fn pin_file_to_ipfs(file: Upload, ctx: &Context<'_>) -> Result<String> {
    let client = Client::new();

    let upload = file.value(ctx).unwrap();
    let filename = upload.filename.clone();
    let mut buff: Vec<u8> = Vec::new();
    let _ = upload.into_read().read_to_end(&mut buff);
    let file_part = reqwest::multipart::Part::bytes(buff)
        .file_name("upload")
        .mime_str("application/octet-stream")
        .unwrap();

    // Create the multipart form
    let form = reqwest::multipart::Form::new()
        .part("file", file_part)
        .text("pinataMetadata", format!(r#"{{"name": "{}"}}"#, filename))
        .text("pinataOptions", r#"{"cidVersion": 1}"#);

    let mut default_headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", Settings::expect_env(EnvVariables::PinyataJwt));
    default_headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_token).unwrap());

    let response = client
        .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
        .headers(default_headers)
        .multipart(form)
        .send()
        .await?;

    let response_text = response.text().await?;

    // Deserialize the response
    let pinata_response: PinataResponse = serde_json::from_str(&response_text)?;

    Ok(pinata_response.IpfsHash)
}

#[derive(Deserialize)]
struct PinataResponse {
    IpfsHash: String,
}
