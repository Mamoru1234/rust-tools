use std::{env, path::PathBuf};

use log::info;
use seahorse::{Command, Context};

use crate::modules::{config_utils::{read_profile, Profile}, git_utils::{ensure_ssh_host, setup_gpg}, spawn_utils::spawn};

pub fn setup_profile(profile: &Profile, working_dir: &PathBuf) {
  info!("Profile config {:?}", profile.email);
  spawn(&format!("git config user.name \"{}\"", profile.name), working_dir).unwrap();
  spawn(&format!("git config user.email \"{}\"", profile.email), working_dir).unwrap();
  if let Some(ssh_host) = &profile.ssh_host {
    ensure_ssh_host("origin", ssh_host, working_dir);
  }
  if let Some(gpg_key) = &profile.gpg {
    setup_gpg(gpg_key, working_dir);
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