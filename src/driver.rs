use clap::ArgMatches;

use crate::reader::Reader;
use crate::writer::Writer;
pub struct Driver;

impl Driver {
    pub fn drive(args: ArgMatches) {
        let (stats, max_width) = Reader::read();
        Writer::write(stats, max_width, args);
    }
}
