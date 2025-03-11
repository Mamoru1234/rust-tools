use std::collections::HashMap;

use tokio::fs;

pub fn get_path() -> String {
  "/home/oleksii/.config/rust-tools/ec2.index.json".to_string()
}

pub async fn read_index() -> Box<HashMap<String, String>> {
  let index_content = String::from_utf8(
    fs::read(get_path()).await
      .expect("Cannot read ec2 index content")
    )
      .expect("Cannot parse utf");
  serde_json::from_str(&index_content).expect("Cannot parse ec2 index")
}
