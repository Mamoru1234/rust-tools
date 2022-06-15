use seahorse::{App, Command, Context};
use serde::{Deserialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs, env};
use std::process;
use std::path::PathBuf;

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
    return config.get(target_profile).expect("Cannot find profile").clone();
}

fn setup_profile(profile: &Profile, working_dir: &PathBuf) {
    println!("Profile config {:?}", profile.email);
    spawn(&format!("git config user.name \"{}\"", profile.name), working_dir).unwrap();
    spawn(&format!("git config core.sshCommand \"ssh -i {} -F /dev/null\"", profile.ssh), working_dir).unwrap();
    println!("Setup ssh {}", profile.ssh);
    spawn(&format!("git config user.email \"{}\"", profile.email), working_dir).unwrap();
    match &profile.gpg {
      Some(gpg_key) => {
        println!("Setup gpg signature {}", gpg_key);
        spawn(&format!("git config user.signingkey {}", gpg_key), working_dir).unwrap();
        spawn("git config commit.gpgsign true", working_dir).unwrap();
      }
      None => {
        println!("No signature provided");
      }
    }
  }

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("my-git-profile")
        .version(env!("CARGO_PKG_VERSION"))
        .usage("my-git-profile [commands]")
        .command(clone_command())
        .command(setup_command());
    app.run(args);
}

fn spawn(command: &str, current_dir: &PathBuf) -> Option<()> {
    let status = process::Command::new("sh")
        .args(&["-c", command])
        .current_dir(current_dir)
        .status()
        .expect("failed to execute command");

    if status.success() {
        return Some(())
    } else {
        println!("Non zero exit code {}", command);
        return None
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
    let current_dir = env::current_dir().expect("cannot get current dir");
    let profile = c.args.get(0).expect("Profile flag is required");
    let profile = read_profile(&profile);
    let repository = c.args.get(1).unwrap();
    println!("Cloning {}", repository);
    spawn(&format!("ssh-agent bash -c \"ssh-add {}; git clone {}\"", profile.ssh, repository), &current_dir).unwrap();
    let project_name = get_dir_name(repository).expect("Cannot get dir name");
    let mut project_dir = current_dir.clone();
    project_dir.push(&project_name);
    println!("cloned into {:?}", &project_dir);
    setup_profile(&profile, &project_dir);
}

fn setup_command() -> Command {
    Command::new("setup")
        .description("setup command")
        .usage("my-git-profile clone [profile]")
        .action(setup_action)
}

fn setup_action(c: &Context) {
    if c.args.len() != 1 {
        println!("Wrong args {:?}", c.args);
        return;
    }
    let profile = c.args.get(0).expect("Profile is required");
    let profile = read_profile(&profile);
    let current_dir = env::current_dir().expect("cannot get current dir");
    setup_profile(&profile, &current_dir);
}
