use std::path::PathBuf;
use crate::locker::{Locker, Encrypted, Distinguished};

/* Args struct */

#[derive(Clone, Debug)]
pub struct Args {
    iterator: u8,
    pub entity: Encrypted,
    pub account: Encrypted,
    pub password: Encrypted,
}

/* Arg struct */

#[derive(Clone, Debug)]
pub struct Arg {
    pub dir: bool,
    pub path: PathBuf,
    pub parent_hash: Option<String>,
    pub values: Distinguished,
}

impl Iterator for Args {
    type Item = Arg;

    fn next(&mut self) -> Option<Self::Item> {
        let mut path = PathBuf::new();

        match self.iterator {
            0 => {
                let dir = true;
                let parent_hash = None;
                let entity_path = self.entity.path();
                let values = self.entity.distinguish();

                self.iterator += 1;

                path.push(entity_path);

                let iteration = Arg {
                    dir,
                    path,
                    parent_hash,
                    values
                };

                Some(iteration)
            },
            1 => {
                if !self.has_account() { return None; }

                let dir = true; 
                let parent_hash = Some(self.entity.hash());
                let entity_path = self.entity.path();
                let account_path = self.account.path();
                let values = self.account.distinguish();

                self.iterator += 1;
                
                path.push(entity_path);
                path.push(account_path);
                
                let iteration = Arg {
                    dir,
                    path,
                    parent_hash,
                    values
                };

                Some(iteration)
            },
            2 => {
                if !self.has_all() { return None; }

                let dir = false;
                let parent_hash = None; 
                let entity_path = self.entity.path();
                let account_path = self.account.path();
                let password_path = self.password.path();
                let values = self.password.distinguish();

                self.iterator = 3;

                path.push(entity_path);
                path.push(account_path);
                path.push(password_path);
               
                let iteration = Arg {
                    dir,
                    path,
                    parent_hash,
                    values
                };

                Some(iteration)
            },
            _ => { None },
        } 
    }
}

impl Args {
    
    /* Initialisers */

    pub fn new(
        entity: Option<&str>,
        account: Option<&str>,
        password: Option<&str>
    ) -> Args {
        let mut locker = Locker::new();
        let mut ent = Encrypted::empty();
        let mut acc = Encrypted::empty();
        let mut pwd = Encrypted::empty();

        if let Some(e) = entity { ent = locker.encrypt(e); }
        if let Some(a) = account { acc = locker.encrypt(a); }
        if let Some(p) = password { pwd = locker.encrypt(p); }

        Args {
            iterator: 0,
            entity: ent,
            account: acc,
            password: pwd
        }
    }

    /* Methods */

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        if self.has_entity() { path.push(self.entity.path()); }
        if self.has_account() { path.push(self.account.path()); }
        if self.has_all() { path.push(self.password.path()); } 

        path 
    } 

    pub fn has_all(&self) -> bool {
        !self.entity.is_empty()
        && !self.account.is_empty()
        && !self.password.is_empty()
    }

    pub fn has_entity(&self) -> bool {
        !self.entity.is_empty()
    }

    pub fn has_account(&self) -> bool {
        !self.account.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        assert_eq!(args.iterator, 0);
        // 0x<encrypted = 34>$<hash = 64>
        assert_eq!(args.entity.path().len(), 99);
        assert_eq!(args.account.path().len(), 99);
        assert_eq!(args.password.path().len(), 99);
    }

    #[test]
    fn path_entity() {
        let args = Args::new(
            Some("entity"),
            None,
            None
        );

        let path = args.path();
        let entity = args.entity.path();

        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(path.to_str().unwrap(), &entity);
    }

    #[test]
    fn path_account() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            None
        );

        let path = args.path();
        let entity = args.entity.path();
        let account = args.account.path();
        let mut test_path = PathBuf::new();

        test_path.push(entity);
        test_path.push(account);

        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(path.to_str().unwrap(), test_path.to_str().unwrap());
    }

    #[test]
    fn path_all() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        let path = args.path();
        let entity = args.entity.path();
        let account = args.account.path();
        let password = args.password.path();
        let mut test_path = PathBuf::new();

        test_path.push(entity);
        test_path.push(account);
        test_path.push(password);

        assert_eq!(path.as_path().iter().count(), 3);
        assert_eq!(path.to_str().unwrap(), test_path.to_str().unwrap());
    }

    #[test]
    fn iterator_entity() {
        let mut args = Args::new(
            Some("entity"),
            None,
            None
        );

        assert_eq!(args.iterator, 0);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat,
            hash
        } = values;
        
        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 1);
    }

    #[test]
    fn iterator_account() {
        let mut args = Args::new(
            Some("entity"),
            Some("account"),
            None
        );

        assert_eq!(args.iterator, 0);

        let Arg { 
            dir,
            path,
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 1);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash.unwrap().len(), 64);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 2);
    }

    #[test]
    fn iterator_password() {
        let mut args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        assert_eq!(args.iterator, 0);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 1);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key,
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash.unwrap().len(), 64);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 2);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key,
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 3);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(dir, false);
        assert_eq!(args.iterator, 3);
    }

    #[test]
    fn iterator_cycle() {
        let mut args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        assert_eq!(args.iterator, 0);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key,
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 1);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash.unwrap().len(), 64);
        assert_eq!(dir, true);
        assert_eq!(args.iterator, 2);

        let Arg { 
            dir,
            path, 
            parent_hash,
            values
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key,
            dat,
            hash
        } = values;

        assert_eq!(iv.len(), 34);
        assert_eq!(key.len(), 34);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.as_path().iter().count(), 3);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(dir, false);
        assert_eq!(args.iterator, 3);

        let finally = args.next();

        assert!(finally.is_none());
    }

    #[test]
    fn has_all() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        let has_all = args.has_all();

        assert_eq!(has_all, true);
    }

    #[test]
    fn has_entity() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        let has_entity = args.has_entity();

        assert_eq!(has_entity, true);
    }

    #[test]
    fn no_entity() {
        let args = Args::new(
            None,
            Some("account"),
            Some("password")
        );

        let has_entity = args.has_entity();

        assert_eq!(has_entity, false);
    }

    #[test]
    fn has_account() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        let has_account = args.has_account();

        assert_eq!(has_account, true);
    }

    #[test]
    fn no_account() {
        let args = Args::new(
            Some("entity"),
            None,
            Some("password")
        );

        let has_account = args.has_account();

        assert_eq!(has_account, false);
    }
}
