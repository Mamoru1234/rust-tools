use std::env;
use std::collections::HashSet;
use std::fs;

fn parse_host_line(line: &str) -> Option<String> {
    match line.split(" ").next() {
        Some(content) => {
            Some(String::from(content))
        },
        None => None,
    }
}

fn drop_hosts(file_path: &String, hosts: &HashSet<String>) -> String {
    fs::read_to_string(file_path)
        .expect("Cannot read hosts file")
        .lines()
        .filter(|line| {
            match parse_host_line(line) {
                Some(line) => {
                    !hosts.contains(&line)
                },
                None => false,
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
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