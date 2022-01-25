pub enum ProcFields {
    Filesystem = 0,
    Mountpoint = 1,
}

impl ProcFields {
    pub fn upcast(self) -> usize {
        self as usize
    }
}
