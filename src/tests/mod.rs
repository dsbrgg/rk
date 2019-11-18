pub mod setup {
    use rand::{Rng, OsRng};

    use std::env;
    use std::path::{Path, PathBuf};

    // https://stackoverflow.com/questions/16946888/is-it-possible-to-make-a-recursive-closure-in-rust
    pub struct Setup<'s> {
        pub paths: Vec<String>,
        pub test: &'s Fn(&mut Self),
        pub after_each: &'s Fn(&mut Self)
    }

    impl<'s> Setup<'s> {
        // TODO: self is not needed here
        fn rand_path(&self) -> String {
            let mut rng = OsRng::new().ok().unwrap();
            let mut rand_str: [u8; 10] = [0; 10];

            rng.fill_bytes(&mut rand_str);

            rand_str 
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .fold(String::from("dump/"), |string, hx| format!("{}{}", string, hx))
        }

        fn gen_path(&mut self) -> PathBuf {
            let path = self.rand_path();
            
            self.paths.push(path.clone());

            let mut dir = env::current_dir().unwrap();

            dir.push(path);

            dir
        }

        // NOTE: maybe Cow<T> can be used here
        pub fn add_to_paths(&mut self, path: String) -> String {
            self.paths.push(format!("dump/{}", path));
            path
        }

        pub fn as_path_buf(&mut self) -> (PathBuf, PathBuf) {
            (
               self.gen_path(), 
               self.gen_path()
            )
        }

        pub fn as_path_str(&mut self) -> (String, String) {
            let (config, locker) = self.as_path_buf();

            (
                config.as_path().to_str().unwrap().to_owned(), 
                locker.as_path().to_str().unwrap().to_owned()
            )
        }
    } 

    // RAII https://stackoverflow.com/questions/38253321/what-is-a-good-way-of-cleaning-up-after-a-unit-test-in-rust
    impl<'s> Drop for Setup<'s> {
        fn drop(&mut self) {
            (self.test)(self);
            (self.after_each)(self); 
        }
    }
}
