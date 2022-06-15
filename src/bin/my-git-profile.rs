use seahorse::{App, Command, Context};
use serde::{Deserialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs, env};
use std::process;

#[derive(Deserialize, Debug, Clone)]
struct Profile {
  email: String,
  name: String,
  ssh: String,
  gpg: Option<String>,
}

fn read_profile(target_profile: &str) -> Profile {
    println!("Target profile {}", target_profile);
    let home = env::var("HOME").expect("No home");
    let contents = fs::read_to_string(format!("{}/.config/rust-tools/config.json", home))
    .expect("Something went wrong reading the file");
    let config: HashMap<String, Profile> = serde_json::from_str(&contents).expect("Cannot parse");
    return config.get(target_profile).unwrap().clone();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("my-git-profile")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("my-git-profile [commands]")
        .command(clone_command());
    let dir_name = "git@github.com:SpotOnInc/wallet-accounts.git";
    println!("get result: {:?}", get_dir_name(&dir_name));
    if dir_name == "git@github.com:SpotOnInc/wallet-accounts.git" {
        return;
    }
    app.run(args);
}

fn spawn(command: &str) -> bool {
    let status = process::Command::new("sh")
        .args(&["-c", command])
        .status()
        .expect("failed to execute command");

    if status.success() {
        return true
    } else {
        println!("Non zero exit code {}", command);
        return false
    }
}

fn clone_command() -> Command {
    Command::new("clone")
        .description("clone command")
        .usage("my-git-profile clone [profile] [repository]")
        .action(clone_action)
}

fn get_dir_name(repository: &str) -> Option<String> {
    if !repository.ends_with(".git") {
        return None;
    }
    let trim_repo: String = repository.chars().take(repository.len() - 4).collect();
    trim_repo.split("/").skip(1).next();
    Some(String::from(trim_repo.split("/").skip(1).next().expect("cannot parse")))
}

fn clone_action(c: &Context) {
    if c.args.len() != 2 {
        println!("Wrong args {:?}", c.args);
        return;
    }
    let profile = c.args.get(0).expect("Profile flag is required");
    let profile = read_profile(&profile);
    println!("Profile: {:?}", profile);
    let repository = c.args.get(1).unwrap();
    println!("Cloning {:?}", repository);
    if !spawn(&format!("ssh-agent bash -c \"ssh-add {}; git clone {}\"", profile.ssh, repository)) {
        return;
    }
}
