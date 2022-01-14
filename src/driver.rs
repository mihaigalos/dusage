use crate::reader::Reader;
use crate::writer::Writer;
pub struct Driver;

impl Driver {
    pub fn drive() {
        let (stats, max_width) = Reader::read();
        Writer::write(stats, max_width);
    }
}
