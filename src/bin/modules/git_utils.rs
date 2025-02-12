use std::path::PathBuf;

use git2::Repository;
use log::info;

const LEGACY_SSH_COMMAND_KEY: &str = "core.sshcommand";

pub fn replace_host(repository: &str, ssh_host: &str) -> String {
  let mut parts = repository.split(":").collect::<Vec<&str>>();
  parts[0] = ssh_host;
  parts.join(":")
}

pub fn ensure_ssh_host(repository: &Repository, remote_name: &str, ssh_host: &str) {
  let remote = repository.find_remote(remote_name).expect("Cannot find remote");
  let remote_url = remote.url().expect("Cannot get remote url");
  let mut config = repository.config().expect("Cannot get config");
  if config.get_entry(LEGACY_SSH_COMMAND_KEY).is_ok() {
    info!("Legacy ssh command detected unset it");
    config.remove(LEGACY_SSH_COMMAND_KEY).expect("Cannot unset ssh command");
  }
  info!("Remote url {}", remote_url);
  if !remote_url.starts_with(ssh_host) {
    let new_origin = replace_host(&remote_url, ssh_host);
    info!("Replacing origin to {}", new_origin);
    repository.remote_set_url(remote_name, &new_origin).expect("Cannot set new remote url");
  }
}

pub fn setup_gpg(repository: &Repository, gpg_key: &str) {
  info!("Setup gpg signature {}", gpg_key);
  let mut config = repository.config().unwrap();
  config.set_str("user.signingkey", gpg_key).expect("Cannot set signing key");
  config.set_bool("commit.gpgsign", true).expect("Cannot set gpg sign");
}

pub fn open_repository(working_dir: &PathBuf) -> Repository {
  match Repository::open(working_dir) {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  }
}
