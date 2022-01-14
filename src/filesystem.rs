pub enum Filesystem {
    Dev = 0,
    Default = 100,
    Network = 200,
    Tempfs = 400,
    Log2Ram = 500,
}

impl Filesystem {
    pub fn new(fs: &str) -> Filesystem {
        match fs {
            "tmpfs" => Filesystem::Tempfs,
            "log2ram" => Filesystem::Log2Ram,
            _ => {
                if fs.starts_with("/dev/") {
                    Filesystem::Dev
                } else if fs.contains(":") {
                    Filesystem::Network
                } else {
                    Filesystem::Default
                }
            }
        }
    }
}
