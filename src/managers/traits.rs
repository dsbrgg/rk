use std::io;
use std::path::{PathBuf};

pub trait Manager {
    fn init(&mut self) -> io::Result<()>;

    fn create(&mut self, path: &str) -> io::Result<()>;

    fn remove(&mut self, path: &str) -> io::Result<()>;

    fn read(&mut self, path: &str) -> io::Result<Vec<String>>;
}
