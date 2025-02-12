use std::path::PathBuf;

use log::info;

use super::spawn_utils::{spawn_output, spawn};


pub fn replace_host(repository: &str, ssh_host: &str) -> String {
  let mut parts = repository.split(":").collect::<Vec<&str>>();
  parts[0] = ssh_host;
  parts.join(":")
}

pub fn get_remote_url(remote: &str, current_dir: &PathBuf) -> Option<String> {
  let output = spawn_output(&format!("git remote get-url {}", remote), current_dir);
  return output.map(|o| {
    o.stdout.trim().to_string()
  });
}

pub fn set_remote_url(remote: &str, current_dir: &PathBuf, new_url: &str) -> Option<()> {
  spawn(&format!("git remote set-url {} {}", remote, new_url), current_dir)
}

pub fn ensure_ssh_host(remote: &str, ssh_host: &str, current_dir: &PathBuf) {
  let origin_url = get_remote_url(remote, current_dir).expect("Cannot get remote url");
  info!("Origin url {}", origin_url);
  if !origin_url.starts_with(ssh_host) {
    let new_origin = replace_host(&origin_url, ssh_host);
    info!("Replacing origin to {}", new_origin);
    set_remote_url(remote, current_dir, &new_origin).expect("Cannot set new remote url");
  }
}

pub fn setup_gpg(gpg_key: &str, current_dir: &PathBuf) {
  info!("Setup gpg signature {}", gpg_key);
  spawn(&format!("git config user.signingkey {}", gpg_key), current_dir).unwrap();
  spawn("git config commit.gpgsign true", current_dir).unwrap();
}