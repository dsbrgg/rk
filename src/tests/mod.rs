pub mod setup {
    use std::env;
    use std::path::PathBuf;
    use std::cell::RefCell;

    // https://stackoverflow.com/questions/16946888/is-it-possible-to-make-a-recursive-closure-in-rust
    pub struct Setup<'s> {
        pub name: &'s str,
        pub test_type: &'s str,
        pub count: (u8, u8),
        pub process: &'s Fn(&Self),
        pub after_each: &'s Fn(&Self)
    }

    impl<'s> Setup<'s> {
        pub fn paths(&self) -> (PathBuf, PathBuf) {
            let mut config_path = env::current_dir().unwrap();
            let mut locker_path = env::current_dir().unwrap();
           
            let lp = format!("dump/{}_{}_{}", self.name, self.test_type, self.count.0);
            let cp = format!("dump/{}_{}_{}", self.name, self.test_type, self.count.1); 

            locker_path.push(lp);
            config_path.push(cp);

            (config_path, locker_path)
        }
    } 

    // RAII https://stackoverflow.com/questions/38253321/what-is-a-good-way-of-cleaning-up-after-a-unit-test-in-rust
    impl<'s> Drop for Setup<'s> {
        // NOTE: current drop impl will only work right now 
        // if the methods `process` and `after_each` and subsequent 
        // methods  remain only needing a `&self`
        fn drop(&mut self) {
            (self.process)(&self);
            (self.after_each)(&self); 
        }
    }
}
