pub enum ProcFields {
    Filesystem = 0,
    Mountpoint = 1,
}

impl ProcFields {
    pub fn downcast(self) -> usize {
        self as usize
    }
}
