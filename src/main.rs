extern crate clap;
extern crate colored;
extern crate nix;

use autoclap::autoclap;
use clap::Arg;
use clap::Command;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app: clap::Command = autoclap!();

    let args = app
        .arg(
            Arg::new("inodes")
                .long("inodes")
                .short('i')
                .help("Display inode information."),
        )
        .arg(
            Arg::new("copy_friendly")
                .long("copy_friendly")
                .short('c')
                .help("Monocrome-friendly background for easy copy-pasting elsewhere."),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    dusage::driver::Driver::drive(args);
}
