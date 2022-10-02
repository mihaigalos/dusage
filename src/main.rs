extern crate clap;
extern crate colored;
extern crate nix;

use autoclap::autoclap;
use clap::Command;
use clap::{Arg, ArgAction};

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
            Arg::new("version")
                .long("version")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("Display version information."),
        )
        .arg(
            Arg::new("copy-friendly")
                .long("copy-friendly")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Monocrome-friendly background for easy copy-pasting elsewhere."),
        );

    let args = app.clone().try_get_matches().unwrap_or_else(|e| e.exit());

    if args.get_flag("version") {
        println!("{}", app.get_name())
    } else {
        dusage::driver::Driver::drive(args);
    }
}
