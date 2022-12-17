use crate::err::CustomError;

pub struct FakerDecryptClient {
}

impl FakerDecryptClient {
  pub async fn new() -> Self {
    Self {} 
  }

  pub async fn decrypt(&self, ciphertext: String) ->Result<String, CustomError> {
    Ok(ciphertext)
  }
}
