use std::{path::PathBuf, process};

use log::info;

pub fn spawn(command: &str, current_dir: &PathBuf) -> Option<()> {
  let status = process::Command::new("sh")
      .args(&["-c", command])
      .current_dir(current_dir)
      .status()
      .expect("failed to execute command");

  if status.success() {
      return Some(())
  } else {
      info!("Non zero exit code {}", command);
      return None
  }
}

#[derive(Debug)]
pub struct SpawnOutput {
  pub stdout: String,
  #[allow(dead_code)]
  pub stderr: String,
}

pub fn spawn_output(command: &str, current_dir: &PathBuf) -> Option<SpawnOutput> {
  let output = process::Command::new("sh")
      .args(&["-c", command])
      .current_dir(current_dir)
      .output()
      .expect("failed to execute command");

  if output.status.success() {
      return Some(SpawnOutput {
        stdout: String::from_utf8(output.stdout).expect("Cannot parse stdout"),
        stderr: String::from_utf8(output.stderr).expect("Cannot parse stderr")
     })
  } else {
      info!("Non zero exit code {}", command);
      return None
  }
}