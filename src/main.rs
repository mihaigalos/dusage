extern crate clap;
extern crate colored;
extern crate nix;

use clap::{clap_app, crate_version};

#[cfg(not(tarpaulin_include))]
fn main() {
    let _args = clap_app!(dusage =>
        (version: crate_version!())
        (author: "Mihai Galos <mihaigalos at gmail dot com>")
        (about: env!("CARGO_PKG_REPOSITORY"))
    )
    .get_matches_safe()
    .unwrap_or_else(|e| e.exit());

    dusage::driver::Driver::drive();
}
