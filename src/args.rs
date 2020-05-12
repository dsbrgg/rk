use std::iter::Iterator;
use std::path::PathBuf;

use crate::locker::{Locker, Distinguished};

#[derive(Clone, Debug)]
pub struct Args {
    pub entity: Values,
    pub account: Values,
    pub password: Values,
    iterator: u8
}

#[derive(Clone, Debug)]
pub struct Arg {
    pub path: PathBuf,
    pub keys: Distinguished,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub is_dir: bool
}

#[derive(Clone, Debug)]
pub struct Values(String, String);

impl Values {
    pub fn get_hash(&self) -> String { self.0.clone() }
    pub fn get_encrypted(&self) -> String { self.1.clone() }
    pub fn is_empty(&self) -> bool { self.get_encrypted().is_empty() }
}

impl Iterator for Args {
    type Item = Arg;

    fn next(&mut self) -> Option<Self::Item> {
        let mut path = PathBuf::new();

        match self.iterator {
            0 => {
                let entity = self.entity.get_encrypted(); 
                let keys = Locker::distinguish(&entity);
                let hash = self.entity.get_hash();
                let parent_hash = None;
                let is_dir = true;

                self.iterator += 1;

                path.push(&keys.dat);

                let iteration = Arg {
                    path,
                    keys,
                    hash,
                    parent_hash,
                    is_dir
                };

                Some(iteration)
            },
            1 => {
                if !self.has_account() {
                    self.iterator = 0;
                    return None;
                }

                let entity = self.entity.get_encrypted();
                let account = self.account.get_encrypted(); 
                let ekeys = Locker::distinguish(&entity);
                let keys = Locker::distinguish(&account);
                let hash = self.account.get_hash();
                let parent_hash = Some(self.entity.get_hash());
                let is_dir = true; 

                self.iterator += 1;
                
                path.push(&ekeys.dat);
                path.push(&keys.dat);
                
                let iteration = Arg {
                    path,
                    keys,
                    hash,
                    parent_hash,
                    is_dir
                };

                Some(iteration)
            },
            2 => {
                if !self.has_all() {
                    return None;
                }

                let entity = self.entity.get_encrypted();
                let account = self.account.get_encrypted();
                let password = self.password.get_encrypted();
                let ekeys = Locker::distinguish(&entity);
                let akeys = Locker::distinguish(&account);
                let keys = Locker::distinguish(&password);
                let hash = self.account.get_hash();
                let parent_hash = None; 
                let is_dir = false;

                self.iterator = 3;

                path.push(&ekeys.dat);
                path.push(&akeys.dat);
                path.push(&keys.dat);
               
                let iteration = Arg {
                    path,
                    keys,
                    hash,
                    parent_hash,
                    is_dir
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
        
        let mut ent = String::new();
        let mut acc = String::new();
        let mut pwd = String::new();

        let mut ehash = String::new();
        let mut ahash = String::new();
        let mut phash = String::new();

        if let Some(e) = entity { 
            ehash = locker.hash(e);
            ent = locker.encrypt(e);
        }
        
        if let Some(a) = account { 
            ahash = locker.hash(a);
            acc = locker.encrypt(a); 
        }

        if let Some(p) = password { 
            phash = locker.hash(p);
            pwd = locker.encrypt(p); 
        }

        Args {
            entity: Values(ehash, ent),
            account: Values(ahash, acc),
            password: Values(phash, pwd),
            iterator: 0
        }
    }

    /* Methods */

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        if self.has_entity() {
            let entity = self.entity.get_encrypted();
            let Distinguished { dat, .. } = Locker::distinguish(&entity);
            path.push(&dat[2..]);
        }

        if self.has_account() {
            let account = self.account.get_encrypted();
            let Distinguished { dat, .. } = Locker::distinguish(&account);
            path.push(&dat[2..]);
        }
        
        if self.has_all() {
            let password = self.password.get_encrypted();
            let Distinguished { dat, .. } = Locker::distinguish(&password);
            path.push(&dat[2..]);
        } 

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
        let locker = Locker::new();
        
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        assert_eq!(args.entity.get_encrypted().len(), 98);
        assert_eq!(args.account.get_encrypted().len(), 98);
        assert_eq!(args.password.get_encrypted().len(), 98);
        assert_eq!(args.iterator, 0);
    }

    #[test]
    fn path_entity() {
        let args = Args::new(
            Some("entity"),
            None,
            None
        );

        let path = args.path();

        assert_eq!(path.as_path().iter().count(), 1);
    }

    #[test]
    fn path_account() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            None
        );

        let path = args.path();

        assert_eq!(path.as_path().iter().count(), 2);
    }

    #[test]
    fn path_all() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        let path = args.path();

        assert_eq!(path.as_path().iter().count(), 3);
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
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;
        
        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(is_dir, true);
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
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(is_dir, true);
        assert_eq!(args.iterator, 1);

        let Arg { 
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 69);
        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash.unwrap().len(), 64);
        assert_eq!(is_dir, true);
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
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(is_dir, true);
        assert_eq!(args.iterator, 1);

        let Arg { 
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 69);
        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash.unwrap().len(), 64);
        assert_eq!(is_dir, true);
        assert_eq!(args.iterator, 2);

        let Arg { 
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 104);
        assert_eq!(path.as_path().iter().count(), 3);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(is_dir, false);
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
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 34);
        assert_eq!(path.as_path().iter().count(), 1);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(is_dir, true);
        assert_eq!(args.iterator, 1);

        let Arg { 
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 69);
        assert_eq!(path.as_path().iter().count(), 2);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash.unwrap().len(), 64);
        assert_eq!(is_dir, true);
        assert_eq!(args.iterator, 2);

        let Arg { 
            path, 
            keys, 
            hash, 
            parent_hash, 
            is_dir 
        } = args.next().unwrap();

        let Distinguished {
            iv,
            key, 
            dat
        } = keys;

        assert_eq!(iv.len(), 32);
        assert_eq!(key.len(), 32);
        assert_eq!(dat.len(), 34);
        assert_eq!(path.to_str().unwrap().len(), 104);
        assert_eq!(path.as_path().iter().count(), 3);
        assert_eq!(hash.len(), 64);
        assert_eq!(parent_hash, None);
        assert_eq!(is_dir, false);
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

