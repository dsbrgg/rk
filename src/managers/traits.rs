use std::io;
use std::path::{PathBuf};

pub trait Manager {
    type Output;

    fn init(&mut self) -> io::Result<()>;

    fn create(&mut self, path: &str) -> io::Result<()>;

    fn remove(&mut self, path: &str) -> io::Result<()>;

    fn read(&mut self, path: &str) -> io::Result<Self::Output>;

    fn pb_to_str(path: &PathBuf) -> String {
        path
            .clone()
            .as_path()
            .to_str()
            .unwrap()
            .to_owned()
    }

    fn append_path(root: &str, paths: &Vec<&str>) -> String {
        let mut path = PathBuf::from(root);
       
        for p in paths.iter() { path.push(p); }

        Self::pb_to_str(&path)
    } 
}
