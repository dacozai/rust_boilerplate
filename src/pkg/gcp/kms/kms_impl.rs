use std::{ collections::HashMap, str };
use serde::{Deserialize};
use reqwest;
use reqwest::header::CONTENT_TYPE;
extern crate base64;
use crate::err::CustomError;
use gcp_auth::AuthenticationManager;

#[derive(Deserialize)]
#[derive(Clone, Debug)]
pub struct PlainResp {
    pub plaintext: String,
    #[serde(rename = "plaintextCrc32c")]
    pub plaintext_crc32c: String,
    #[serde(rename = "usedPrimary")]
    pub used_primary: bool,
    #[serde(rename = "protectionLevel")]
    pub protection_level: String,
}

pub struct KMSClient {
  pub project_id: String,
  pub location: String,
  pub key_ring: String, 
  pub key: String,
  access_token: String,
}

impl KMSClient {
  pub async fn new(project_id: String, location: String, key_ring: String, key: String) -> Self {
    let authentication_manager = AuthenticationManager::new().await.unwrap();
    let token = authentication_manager
        .get_token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await
        .expect("failed to get access_token from the default application account");
    
    Self { 
      project_id: project_id, 
      location: location, 
      key_ring: key_ring, 
      key: key, 
      access_token: token.as_str().to_string(),
    } 
  }

  pub async fn decrypt(&self, ciphertext: String) ->Result<String, CustomError> {
    let mut d: HashMap<&str, String> = HashMap::new();
    let client = reqwest::Client::new();
    let uri = format!("https://cloudkms.googleapis.com/v1/\
      projects/{}/locations/{}/keyRings/{}/cryptoKeys/{}:decrypt",
      self.project_id,
      self.location,
      self.key_ring,
      self.key,
    );

    d.insert("ciphertext", ciphertext);
    let txt = client.post(uri)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .bearer_auth(&self.access_token)
        .json(&d)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to read text");
    let res = serde_json::from_str::<PlainResp>(&txt)
      .expect("failed to parse response body into PlainResp struct");
    let bytes_str = base64::decode(res.plaintext).unwrap();
    let plaintext = str::from_utf8(&bytes_str).unwrap().replace("\n", "");
    // println!("{}", pp);
    Ok(plaintext)
  }
}
