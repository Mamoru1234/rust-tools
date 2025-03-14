use seahorse::Command;

mod index_command;
mod start_command;
mod stop_command;
use index_command::index_command;
use start_command::start_command;
use stop_command::stop_command;

pub fn ec2_command() -> Command {
  Command::new("ec2")
    .description("EC2 manipulations command")
    .command(index_command())
    .command(start_command())
    .command(stop_command())
}
