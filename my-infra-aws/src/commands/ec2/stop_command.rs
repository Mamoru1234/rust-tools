use log::info;
use seahorse::{Command, Context};
use tokio::runtime::Runtime;

use crate::utils::ec2_index::read_index;

async fn stop_action_async(ctx: &Context) {
  let service_name: String = ctx.args.get(0).expect("Service name is required").to_string();
  info!("Stoping {}", &service_name);
  let index = read_index().await;
  let instance_id = index.get(&service_name).expect("Service is unknown");
  let config = aws_config::from_env().profile_name("personal").region("eu-central-1").load().await;
  let client = aws_sdk_ec2::Client::new(&config);
  client.stop_instances().instance_ids(instance_id).send().await.unwrap();
}

fn stop_action(ctx: &Context) {
  Runtime::new().unwrap().block_on(stop_action_async(ctx));
}

pub fn stop_command() -> Command {
  Command::new("ec2:stop").description("Stop ec2 instance").action(stop_action)
}