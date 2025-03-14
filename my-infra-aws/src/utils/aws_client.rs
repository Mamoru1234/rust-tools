use dotenv_codegen::dotenv;

pub async fn get_ec2_client() -> Box<aws_sdk_ec2::Client> {
  let config = aws_config::from_env().profile_name(dotenv!("AWS_PROFILE")).region(dotenv!("AWS_REGION")).load().await;
  Box::new(aws_sdk_ec2::Client::new(&config))
}

pub async fn get_ecr_client() -> Box<aws_sdk_ecr::Client> {
  let config = aws_config::from_env().profile_name(dotenv!("AWS_PROFILE")).region(dotenv!("AWS_REGION")).load().await;
  Box::new(aws_sdk_ecr::Client::new(&config))
}
