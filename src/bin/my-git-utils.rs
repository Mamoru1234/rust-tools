use commands::{clone_command, setup_command};
use seahorse:: App;
use std::env;

mod modules;
mod commands;

fn main() {
  env_logger::init();
  let args: Vec<String> = env::args().collect();
  let app = App::new("my-git-utils")
      .version(env!("CARGO_PKG_VERSION"))
      .usage("my-git-utils [commands]")
      .command(clone_command())
      .command(setup_command());
  app.run(args);
}
