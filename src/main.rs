#![warn(clippy::all, clippy::pedantic)]

use rust_github_actions_test::foo;

fn main() {
    println!(
        "Hello, world V2!: {:?} {:?}",
        std::path::MAIN_SEPARATOR,
        std::env::current_dir().unwrap(),
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
