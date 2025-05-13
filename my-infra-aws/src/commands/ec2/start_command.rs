use std::time::Duration;

use aws_sdk_ec2::client::Waiters;
use log::info;
use seahorse::{Command, Context};
use tokio::runtime::Runtime;
use nix::unistd::Uid;

use crate::utils::{aws_client::get_ec2_client, ec2_index::read_index, hosts::replace_host_ip};

async fn wait_for_instance_ready(
  client: &aws_sdk_ec2::Client,
  instance_id: &str,
) {
  client
      .wait_until_instance_status_ok()
      .instance_ids(instance_id)
      .wait(Duration::from_secs(600))
      .await
      .unwrap();
}

async fn get_instance_public_ip(
  client: &aws_sdk_ec2::Client,
  instance_id: &str,) -> String {
  let describe_output = client.describe_instances().instance_ids(instance_id).send().await.unwrap();
  let ip_address = describe_output.reservations()[0].instances()[0].public_ip_address().unwrap();
  ip_address.to_string()
}

async fn start_action_async(ctx: &Context) {
  let service_name: String = ctx.args.get(0).expect("Service name is required").to_string();
  info!("Starting {}", &service_name);
  if !cfg!(debug_assertions) && !Uid::effective().is_root() {
    panic!("Script should be executed as root")
  }
  let index = read_index().await;
  let instance_id = index.get(&service_name).expect("Service is unknown");
  let client = get_ec2_client().await;
  info!("Wait for instance stopped");
  client.wait_until_instance_stopped().instance_ids(instance_id).wait(Duration::from_secs(600)).await.unwrap();
  info!("Starting instance");
  client.start_instances().instance_ids(instance_id).send().await.unwrap();
  info!("Waiting for instance ok");
  wait_for_instance_ready(&client, &instance_id).await;
  info!("Instance started");
  let public_ip = get_instance_public_ip(&client, &instance_id).await;
  info!("Public IP of started instance {}", &public_ip);
  replace_host_ip(&service_name, &public_ip);
}

fn start_action(ctx: &Context) {
  Runtime::new().unwrap().block_on(start_action_async(ctx));
}

pub fn start_command() -> Command {
  Command::new("start").description("Start ec2 instance").action(start_action)
}