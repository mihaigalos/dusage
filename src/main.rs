extern crate clap;
extern crate colored;
extern crate nix;

use autoclap;
use clap::App;
use clap::Arg;
use std::env;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app: clap::App<'static> = autoclap::autoclap!();
    let args = app
        .arg(
            Arg::new("inodes")
                .long("inodes")
                .short('i')
                .help("Display inode information."),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    dusage::driver::Driver::drive(args);
}
