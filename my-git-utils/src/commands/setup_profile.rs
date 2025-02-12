use std::{env, path::PathBuf};

use log::info;
use seahorse::{Command, Context};

use crate::modules::{config_utils::{read_profile, Profile}, git_utils::{ensure_ssh_host, open_repository, setup_gpg}};

pub fn setup_profile(profile: &Profile, working_dir: &PathBuf) {
  info!("Profile config {:?}", profile.email);
  let repo = open_repository(working_dir);
  repo.config().unwrap().set_str("user.name", &profile.name).unwrap();
  repo.config().unwrap().set_str("user.email", &profile.email).unwrap();
  if let Some(ssh_host) = &profile.ssh_host {
    ensure_ssh_host(&repo, "origin", ssh_host);
  }
  if let Some(gpg_key) = &profile.gpg {
    setup_gpg(&repo, gpg_key);
  }
}


pub fn setup_command() -> Command {
  Command::new("setup")
      .alias("sp")
      .description("setup command")
      .usage("my-git-profile clone [profile]")
      .action(setup_action)
}

fn setup_action(c: &Context) {
  if c.args.len() != 1 {
      info!("Wrong args {:?}", c.args);
      return;
  }
  let profile = c.args.get(0).expect("Profile is required");
  let profile = read_profile(&profile);
  let current_dir = env::current_dir().expect("cannot get current dir");
  setup_profile(&profile, &current_dir);
}