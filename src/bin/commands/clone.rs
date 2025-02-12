use std::env;

use log::info;
use seahorse::{Command, Context};
use super::setup_profile::setup_profile;
use crate::modules::{config_utils::read_profile, git_utils::replace_host, spawn_utils::spawn};

pub fn clone_command() -> Command {
  Command::new("clone")
    .alias("cp")
    .description("clone command")
    .usage("my-git-profile clone [profile] [repository]")
    .action(clone_action)
}

fn get_project_name(repository: &str) -> Option<String> {
  if !repository.ends_with(".git") {
      return None;
  }
  let trim_repo: String = repository.chars().take(repository.len() - 4).collect();
  Some(String::from(trim_repo.split("/").last().expect("cannot parse")))
}

fn clone_action(c: &Context) {
  if c.args.len() != 2 {
      info!("Wrong args {:?}", c.args);
      return;
  }
  let current_dir = env::current_dir().expect("cannot get current dir");
  let profile = c.args.get(0).expect("Profile flag is required");
  let profile = read_profile(profile);
  info!("Profile {:?}", profile);
  let repository = c.args.get(1).unwrap();
  info!("Cloning {}", repository);
  let project_name = get_project_name(repository).expect("Cannot get project name");
  let project_dir = current_dir.join(project_name);
  match &profile.ssh_host {
      Some(ssh_host) => {
          spawn(&format!("git clone {}", replace_host(repository, &ssh_host)), &current_dir).unwrap();
      }
      None => {
          spawn(&format!("git clone {}", repository), &current_dir).unwrap();
      }
  }
  info!("Cloned into {:?}", &project_dir);
  setup_profile(&profile, &project_dir);
}
