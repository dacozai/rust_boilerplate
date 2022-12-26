#[cfg(test)]
mod decrypter_tests {
    use super::super::{*};

    #[tokio::test]
    async fn module_private_test() {
      let p = String::from("check me out");
      let s = FakerDecryptClient::new().await;
      let plaintext = s.decrypt(String::from("check me out"))
        .await
        .expect("unwrap test failed");
      assert!(
        p.eq(&plaintext),
        "Failed to get the decrypted text, got `{}`", plaintext,
      );
    }
}