use std::env;

pub mod commands;
pub mod utils;

use commands::ec2;
use dotenv::dotenv;
use env_logger::{Env, Builder};
use seahorse::App;

fn main() {
    dotenv().ok();
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .command(ec2::index_command())
        .command(ec2::start_command())
        .command(ec2::stop_command());
    app.run(args);
}
