use std::fs;

use log::warn;
use nix::unistd::Uid;

fn get_hosts_path() -> String {
  if cfg!(debug_assertions) {
    return String::from("./test-hosts");
  }
  return String::from("/etc/hosts");
}

fn replace_line_ip(line: &str, ip_addr: &str) -> String {
  let space_ind = line.find(char::is_whitespace).expect("Cannot find whitespace in hst line");
  ip_addr.to_string() + &line[space_ind..]
}

pub fn replace_host_ip(service_name: &str, ip_addr: &str) {
  if !cfg!(debug_assertions) && !Uid::effective().is_root() {
    panic!("Script should be executed as root")
  }
  let service_anchor = "#EC2Instance: ".to_owned() + service_name;
  let hosts_path = get_hosts_path();
  let hosts = fs::read_to_string(&hosts_path).unwrap();
  let mut new_hosts: Vec<String> = Vec::new();
  let mut is_replaced = false;
  for line in hosts.lines() {
    if !line.contains(&service_anchor) {
      new_hosts.push(line.to_string());
      continue;
    } else {
      is_replaced = true;
      new_hosts.push(replace_line_ip(line, ip_addr));
    }
  }
  if !is_replaced {
    warn!("Service was not replaced {}", service_name);
    return;
  }
  fs::write(&hosts_path, new_hosts.join("\n")).unwrap()
}