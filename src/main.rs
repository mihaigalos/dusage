extern crate clap;
extern crate colored;
extern crate nix;
use clap::{App, Arg};

#[cfg(not(tarpaulin_include))]
fn main() {
    let args = App::new(concat!(
        env!("CARGO_CRATE_NAME"),
        " ",
        env!("CARGO_PKG_VERSION"),
        " :: ",
        env!("CARGO_PKG_REPOSITORY")
    ))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(
        Arg::with_name("inodes")
            .long("inodes")
            .short("i")
            .help("Display inode information."),
    )
    .get_matches_safe()
    .unwrap_or_else(|e| e.exit());

    dusage::driver::Driver::drive(args);
}
