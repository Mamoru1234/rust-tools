use std::{collections::HashMap, path::PathBuf};

use log::info;

use super::spawn_utils::{spawn_output, spawn};

const LEGACY_SSH_COMMAND_KEY: &str = "core.sshcommand";

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

pub fn unset_config(key: &str, current_dir: &PathBuf) -> Option<()> {
  spawn(&format!("git config --unset {}", key), current_dir)
}

pub fn ensure_ssh_host(remote: &str, ssh_host: &str, current_dir: &PathBuf) {
  let origin_url = get_remote_url(remote, current_dir).expect("Cannot get remote url");
  let git_config = read_config(current_dir);
  if git_config.contains_key(LEGACY_SSH_COMMAND_KEY) {
    info!("Legacy ssh command detected unset it");
    unset_config(LEGACY_SSH_COMMAND_KEY, current_dir).expect("Cannot unset ssh command");
  }
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

pub fn read_config(current_dir: &PathBuf) -> HashMap<String, String> {
  let output = spawn_output("git config --list", current_dir).expect("Cannot read git config");
  let mut config_map = HashMap::new();
  for line in output.stdout.lines() {
      let parts: Vec<&str> = line.trim().split('=').collect();
      if parts.len() == 2 {
          config_map.insert(parts[0].to_string(), parts[1].to_string());
      }
  }
  return config_map;
}