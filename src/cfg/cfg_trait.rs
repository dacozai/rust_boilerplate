
use crate::pkg::faker::FakerDecryptClient;
use crate::pkg::gcp::kms::KMSClient;
use crate::err::CustomError;
use async_trait::async_trait;

#[async_trait]
pub trait Decrypter {
  async fn decrypt(&self, ciphertext: String) ->Result<String, CustomError>;
}

#[async_trait]
impl Decrypter for FakerDecryptClient {
  async fn decrypt(&self, ciphertext: String) ->Result<String, CustomError> {
    self.decrypt(ciphertext).await
  }
}

#[async_trait]
impl Decrypter for KMSClient {
  async fn decrypt(&self, ciphertext: String) ->Result<String, CustomError> {
    self.decrypt(ciphertext).await
  }
}
