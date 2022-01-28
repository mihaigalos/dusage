extern crate clap;
extern crate colored;
extern crate nix;

use autoclap::autoclap;
use clap::Arg;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app: clap::App<'static> = autoclap();
    let args = app
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
