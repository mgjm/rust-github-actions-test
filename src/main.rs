#![warn(clippy::all, clippy::pedantic)]

use std::{env, path::PathBuf};

use clap::Parser;
use rust_github_actions_test::foo;

#[derive(Parser)]
/// Upload a configuration with up to 8 messages to an LED badge
#[clap(
    version = rust_github_actions_test::cli::VERSION,
    author,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
    ",
)]
struct Args {
    /// File format of the config file (toml, json)
    #[clap(long)]
    format: Option<String>,

    /// Path to TOML configuration file
    config: PathBuf,
}

fn main() {
    let _args = Args::parse();

    println!(
        "Hello, world V2!: {:?} {:?}",
        std::path::MAIN_SEPARATOR,
        env::current_dir().unwrap(),
    );

    foo();

    match hidapi::HidApi::new() {
        Ok(hidapi) => {
            println!("devices:");
            for device in hidapi.device_list() {
                println!(
                    "- vendor={:?} {:?} product={:?} {:?} serial={:?} path={:?}",
                    device.vendor_id(),
                    device.manufacturer_string(),
                    device.product_id(),
                    device.product_string(),
                    device.serial_number(),
                    device.path(),
                );
            }
            println!("--- done");
        }
        Err(err) => eprintln!("Error: {err:?}"),
    }
}
