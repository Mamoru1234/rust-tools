mod cleanup_command;
use cleanup_command::cleanup_command;
use seahorse::Command;

pub fn ecr_command() -> Command {
  Command::new("ecr").description("ECR manipulations").command(cleanup_command())
}