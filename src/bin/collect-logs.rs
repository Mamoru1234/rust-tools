use std::{fs, env};
use std::collections::HashSet;
use std::path::PathBuf;
use std::io::Error;
use std::process::{Command, Stdio};


fn collect_logs(container: &String, logs_dir: &PathBuf) -> Result<(), Error> {
    let mut container_logs = logs_dir.clone();
    container_logs.push(format!("{}.log", container));
    let container_logs = fs::File::create(container_logs).expect("Cannot open logs file");
    let container_errors = container_logs.try_clone()?;

    Command::new("sh")
        .args(&["-c", &format!("docker logs {}", &container)])
        .stdout(Stdio::from(container_logs))
        .stderr(Stdio::from(container_errors))
        .spawn()?
        .wait_with_output()?;
    Ok(())
}

fn main() {
    let containers: HashSet<String> = env::args().skip(1).collect();
    let logs_dir = env::var("COLLECT_LOG_PATH").unwrap_or(String::from("."));
    if containers.is_empty() {
        println!("no containers to collect");
        return;
    }
    println!("Collect logs from {:?} ", containers);
    let logs_dir = PathBuf::from(logs_dir);
    if !logs_dir.exists() {
        println!("Creating log dir");
        fs::create_dir_all(&logs_dir).expect("Cannot create dir");
    }
    let logs_dir = fs::canonicalize(&logs_dir).expect("No dir");
    println!("Log dir: {:?}", logs_dir);
    for container in containers {
        collect_logs(&container, &logs_dir).expect("Cannot collect logs");
    }
}
