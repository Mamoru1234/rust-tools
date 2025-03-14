use aws_sdk_ecr::types::ListImagesFilter;
use log::info;
use seahorse::{Command, Context};
use tokio::runtime::Runtime;

use crate::utils::aws_client::get_ecr_client;

async fn cleanup_action_async(_ctx: &Context) {
  info!("Cleaning ECR repositories");
  let client = get_ecr_client().await;
  let repositories = client.describe_repositories().into_paginator().items().send().try_collect().await.expect("Cannot get repositories");
  for repository in &repositories {
    let repository_name = repository.repository_name().unwrap();
    if repository.image_tag_mutability().is_some_and(|tag| *tag != "MUTABLE".into()) {
      info!("Skip cleanup since repository is immutable {}", repository_name);
      continue;
    }
    let untagged_images: Vec<_> = client.list_images()
      .filter(ListImagesFilter::builder()
        .tag_status("UNTAGGED".into()).build())
      .repository_name(repository_name)
      .into_paginator().items().send().try_collect().await.expect("Cannot list images");
    if untagged_images.is_empty() {
      info!("Skip delete since no untagged images for {}", repository_name);
      continue;
    }
    info!("Found untagged images {} [{}]", repository_name, untagged_images.len());
    client.batch_delete_image()
      .repository_name(repository.repository_name().unwrap())
      .set_image_ids(Some(untagged_images))
      .send().await.expect("Cannot delete images");
  }
  info!("Cleanup completed");
}

fn cleanup_action(ctx: &Context) {
  Runtime::new().unwrap().block_on(cleanup_action_async(ctx));
}

pub fn cleanup_command() -> Command {
  Command::new("cleanup").alias("cu").action(cleanup_action)
}