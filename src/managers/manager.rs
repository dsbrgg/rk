use std::io;
use std::path::{PathBuf};

pub trait Manager {
    type Output;

    fn init(&mut self) -> io::Result<()>;

    fn create(&mut self, path: &str) -> io::Result<()>;

    fn remove(&mut self, path: &str) -> io::Result<()>;

    fn read(&mut self, path: &str) -> io::Result<Self::Output>;

    fn write(&mut self, path: &str, content: &str) -> io::Result<()> { Ok(()) }

    fn pb_to_str(path: &PathBuf) -> String {
        path
            .as_path()
            .to_str()
            .unwrap()
            .to_owned()
    }

    fn append_paths(root: &str, paths: &Vec<&str>) -> String {
        let mut buf = PathBuf::from(root);
       
        for path in paths.iter() { 
            if !path.is_empty() { buf.push(path); }
        }

        Self::pb_to_str(&buf)
    }

    fn append_path(root: &str, path: &str) -> String {
        let mut buf = PathBuf::from(root);
      
        if !path.is_empty() { buf.push(path); }

        Self::pb_to_str(&buf)
    }
}
