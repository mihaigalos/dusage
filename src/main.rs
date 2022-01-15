extern crate clap;
extern crate colored;
extern crate nix;
use clap::App;

#[cfg(not(tarpaulin_include))]
fn main() {
    let _matches = App::new(concat!(
        env!("CARGO_CRATE_NAME"),
        " ",
        env!("CARGO_PKG_VERSION"),
        " :: ",
        env!("CARGO_PKG_REPOSITORY")
    ))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .get_matches_safe()
    .unwrap_or_else(|e| e.exit());

    dusage::driver::Driver::drive();
}
