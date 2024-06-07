#![warn(clippy::all, clippy::pedantic)]

#[cfg(feature = "example")]
#[cfg(target_feature = "crt-static")]
pub fn foo() {
    println!("the C runtime should be statically linked");
}

#[cfg(feature = "example")]
#[cfg(not(target_feature = "crt-static"))]
pub fn foo() {
    println!("the C runtime should be dynamically linked");
}

#[cfg(feature = "cli")]
pub mod cli {
    include!(concat!(env!("OUT_DIR"), "/cli.rs"));
}
