use std::{collections::HashMap, env, fs};

use log::info;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
  pub email: String,
  pub name: String,
  pub ssh_host: Option<String>,
  pub gpg: Option<String>,
}

pub fn read_profile(target_profile: &str) -> Profile {
    info!("Target profile {}", target_profile);
    let home = env::var("HOME").expect("No home");
    let contents = fs::read_to_string(format!("{}/.config/rust-tools/config.json", home))
        .expect("Something went wrong reading the file");
    let config: HashMap<String, Profile> = serde_json::from_str(&contents).expect("Cannot parse");
    return config.get(target_profile).expect("Cannot find profile").clone();
}
