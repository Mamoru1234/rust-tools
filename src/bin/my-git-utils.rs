use commands::{clone_command, clean_up_command, setup_command};
use env_logger::Env;
use seahorse:: App;
use std::env;

mod modules;
mod commands;

fn main() {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
  let args: Vec<String> = env::args().collect();
  let app = App::new("my-git-utils")
      .version(env!("CARGO_PKG_VERSION"))
      .usage("my-git-utils [commands]")
      .command(clone_command())
      .command(setup_command())
      .command(clean_up_command());
  app.run(args);
}
