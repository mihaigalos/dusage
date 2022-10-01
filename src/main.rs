extern crate clap;
extern crate colored;
extern crate nix;

use autoclap::autoclap;
use clap::{Arg, ArgAction};
use clap::Command;

#[cfg(not(tarpaulin_include))]
fn main() {
    let app: clap::Command = autoclap!()
        .arg(
            Arg::new("inodes")
                .long("inodes")
                .short('i')
                .help("Display inode information."),
        )
        .arg(
            Arg::new("copy-friendly")
                .long("copy-friendly")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Monocrome-friendly background for easy copy-pasting elsewhere."),
        );
    let args = app.try_get_matches().unwrap_or_else(|e| e.exit());

    dusage::driver::Driver::drive(args);
}
