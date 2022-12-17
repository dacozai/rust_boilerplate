use config::{Config, File};
use std::{ env, collections::HashMap, string::String, collections::HashSet };
use std::sync::Mutex;
use crate::cfg::Decrypter;
use crate::pkg::faker::FakerDecryptClient;

lazy_static::lazy_static! {
  #[derive(Debug)]
  static ref SETTINGS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub async fn init_config() {
  // TODO: build the skip set into lazy static which is considered as better implementation
  // as it is a pre-set variable instead of following the init function
  let skip_set: HashSet<String> = HashSet::from([
      String::from("port"), 
      String::from("host"), 
      String::from("log_level"), 
    ]);

  let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
  let s = Config::builder()
    .add_source(
      match run_mode.as_str() {
        "stag" => File::with_name("src/cfg/stag"),
        "prod" => File::with_name("src/cfg/prod"),
        _ => File::with_name("src/cfg/dev"),
      }
    )
    .build()
    .unwrap()
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();
 
  let decrypter: Box<dyn Decrypter> = Box::new(FakerDecryptClient::new().await);

  for (k,v) in &s {
    SETTINGS
    .lock()
    .unwrap()
    .insert(
      k.to_string(), 
      if skip_set.contains(k) {
        v.clone()
      } else {
        decrypter.decrypt(v.to_string()).await.unwrap()
      }
    );
  }
}

pub fn read(key: &str) -> String {
  SETTINGS
    .lock()
    .unwrap()
    .get(key)
    .unwrap()
    .to_string()
}
