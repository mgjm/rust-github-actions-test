#![warn(clippy::all, clippy::pedantic)]

fn main() {
    println!(
        "Hello, world!: {:?} {:?}",
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

#[cfg(target_feature = "crt-static")]
fn foo() {
    println!("the C runtime should be statically linked");
}

#[cfg(not(target_feature = "crt-static"))]
fn foo() {
    println!("the C runtime should be dynamically linked");
}
