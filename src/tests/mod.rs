pub mod setup {
    use std::env;
    use std::path::PathBuf;
    
    pub struct Setup<'s> {
        pub name: &'s str,
        pub test_type: &'s str,
        pub count: (u8, u8),
        pub drop: bool
    }

    impl<'s> Setup<'s> { 
        pub fn paths(&mut self) -> (PathBuf, PathBuf) {
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
        fn drop(&mut self) {
            use std::fs::remove_dir_all;

            if self.drop {
                let locker_path = format!("dump/{}_{}_{}", self.name, self.test_type, self.count.0);
                let config_path = format!("dump/{}_{}_{}", self.name, self.test_type, self.count.1);

                remove_dir_all(locker_path)
                    .expect("Could not remove file in test");
                remove_dir_all(config_path)
                    .expect("Could not remove file in test");
            } 
        }
    }
}
