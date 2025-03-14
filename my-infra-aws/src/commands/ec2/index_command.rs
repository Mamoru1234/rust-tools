
use std::collections::HashMap;

use aws_sdk_ec2::types::Instance;
use log::info;
use seahorse::{Command, Context};
use tokio::{fs, runtime::Runtime};

use crate::utils::{aws_client::get_ec2_client, ec2_index::get_path};

fn get_instance_name(instance: &Instance) -> Option<String> {
    let name_tag= instance.tags().iter()
        .find(|tag| tag.key().is_some_and(| tag_key| tag_key == "Name"));
    Some(name_tag?.value()?.to_string())
}

async fn get_index(client: &aws_sdk_ec2::Client) -> Box<HashMap<String, String>> {
    let describe_output = client.describe_instances().into_paginator().items().send().try_collect().await.expect("Cannot get instances");
    Box::new(describe_output
        .iter()
        .flat_map(|it| it.instances())
        .filter_map(|instance| {
            Some((
                get_instance_name(instance)?,
                instance.instance_id()?.to_string(),
            ))
        })
        .collect())
}

async fn index_action_async() {
  info!("EC2 async index action");
  let client = get_ec2_client().await;
  let index=  get_index(&client).await;
  info!("Index builded");
  let index_json = serde_json::to_string(&index).expect("Cannot convert index to string");
  fs::write(get_path(), index_json).await.expect("Cannot save index");
}

fn index_action(_: &Context) {
  Runtime::new().unwrap().block_on(index_action_async());
}

pub fn index_command() -> Command {
  Command::new("index")
    .description("Index ec2 instances")
    .action(index_action)
}