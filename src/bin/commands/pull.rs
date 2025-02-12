use std::env;

use git2::{RepositoryState, StashFlags};
use log::info;
use seahorse::{Command, Context};

use crate::modules::{git_utils::{check_has_changes, open_repository}, spawn_utils::spawn};

fn pull_action(_c: &Context) {
  info!("Pulling");
  let target_repository = env::current_dir().unwrap();
  let mut repository = open_repository(&target_repository);
  let repo_state = repository.state();
  if repo_state != RepositoryState::Clean {
    panic!("Repository is not clean");
  }
  let signature = repository.signature().expect("Cannot get signature");
  let has_changes = check_has_changes(&repository);
  if has_changes{
    repository.stash_save2(&signature, None, Some(StashFlags::INCLUDE_UNTRACKED)).expect("Cannot save stash");
  }
  spawn("git pull -r", &target_repository).expect("Cannot pull repository");
  if has_changes {
    repository.stash_pop(0, None).expect("Cannot pop stash");
  }
}

pub fn pull_command() -> Command {
  Command::new("pull")
      .alias("pl")
      .action(pull_action)
}