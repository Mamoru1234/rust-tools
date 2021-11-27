use std::env;
use std::collections::HashSet;
use std::fs;
use dialoguer;

fn parse_host_line(line: &str) -> Option<String> {
    match line.split(" ").next() {
        Some(content) => {
            Some(String::from(content))
        },
        None => None,
    }
}

fn ask_for_host(host: &str) -> bool {
    let input : String = dialoguer::Input::new()
        .with_prompt(format!("Want to remove {}", host))
        .interact_text()
        .expect("cannot interact");
    input == "y"
}

fn drop_hosts(file_path: &String, hosts: &HashSet<String>) -> String {
    let file_content = fs::read_to_string(file_path)
        .expect("Cannot read hosts file");
    let lines = file_content.lines();
    let mut result: Vec<&str> = Vec::new();
    for line in lines {
        let host = parse_host_line(&line);
        if host.is_none() {
            result.push(line);
            continue;
        }
        let host = host.unwrap();
        if !hosts.contains(&host) {
            result.push(line);
            continue;
        }
        if !ask_for_host(&host) {
            result.push(line);
        }
    }
    result.join("\n")
}

fn main() {
    let hosts: HashSet<String> = env::args().skip(1).collect();
    if hosts.is_empty() {
        println!("no hosts to filter");
        return;
    }
    let home_path = env::var("HOME").expect("No home env detected");
    let file_path = format!("{}/.ssh/known_hosts", home_path);
    println!("Drop known hosts {:?} in {} ", hosts, file_path);
    let result = drop_hosts(&file_path, &hosts);
    fs::write(&file_path, result.as_bytes()).expect("cannot write content");
}