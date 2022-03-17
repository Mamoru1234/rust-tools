use serde::{Deserialize};
use std::{fs, env};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::process::{Command, ExitStatus};

#[derive(Deserialize, Debug)]
struct Profile {
  email: String,
  name: String,
  ssh: Option<String>
}

fn spawn(command: &str) -> io::Result<ExitStatus> {
  return Command::new("sh")
    .args(&["-c", command])
    .status();
}

fn setup_profile(profile: &Profile) {
  println!("Profile config {:?}", profile.email);
  spawn(&format!("git config user.name \"{}\"", profile.name)).unwrap();
  spawn(&format!("git config user.email \"{}\"", profile.email)).unwrap();
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("args {:?}", args);
  if args.len() != 2 {
    println!("Wrong number of arguments");
    return;
  }
  let target_profile = args.get(1).unwrap();
  println!("Target profile {}", target_profile);
  let home = env::var("HOME").expect("No home");
  let contents = fs::read_to_string(format!("{}/.config/rust-tools/config.json", home))
    .expect("Something went wrong reading the file");
  let config: HashMap<String, Profile> = serde_json::from_str(&contents).expect("Cannot parse");
  let profile = config.get(target_profile);
  match profile {
    Some(profile) => setup_profile(profile),
    None => println!("Profile not found")
  }
}
