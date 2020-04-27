use std::iter::Iterator;
use std::path::PathBuf;

use crate::locker::{Locker, Distinguished};

#[derive(Clone, Debug)]
pub struct Args {
    pub entity: String,
    pub account: String,
    pub password: String,
    iterator: u8
}

impl Iterator for Args {
    type Item = (PathBuf, Distinguished, bool);

    fn next(&mut self) -> Option<(PathBuf, Distinguished, bool)> {
        let mut path = PathBuf::new();

        match self.iterator {
            0 => {
                let distinguished = Locker::distinguish(&self.entity);

                self.iterator += 1;
                path.push(&self.entity); 

                Some((path, distinguished, true))
            },
            1 => {
                if !self.has_account() {
                    self.iterator = 0;
                    return None;
                }

                self.iterator += 1;
                path.push(&self.entity);
                path.push(&self.account);
                
                let distinguished = Locker::distinguish(&self.account);

                Some((path, distinguished, true))
            },
            2 => {
                self.iterator = 3;

                if !self.has_all() {
                    return None;
                }

                path.push(&self.entity);
                path.push(&self.account);
                path.push(&self.password);
                
                let distinguished = Locker::distinguish(&self.password);

                Some((path, distinguished, false))
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

        if let Some(e) = entity { ent = locker.encrypt(e); }
        if let Some(a) = account { acc = locker.encrypt(a); }
        if let Some(p) = password { pwd = locker.encrypt(p); }

        Args {
            entity: ent,
            account: acc,
            password: pwd,
            iterator: 0
        }
    }

    /* Methods */

    pub fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();

        if self.has_entity() {
            let Distinguished { dat, .. } = Locker::distinguish(&self.entity);
            path.push(&dat[2..]);
        }

        if self.has_account() {
            let Distinguished { dat, .. } = Locker::distinguish(&self.account);
            path.push(&dat[2..]);
        }
        
        if self.has_all() {
            let Distinguished { dat, .. } = Locker::distinguish(&self.password);
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

        assert_eq!(args.entity.len(), 98);
        assert_eq!(args.account.len(), 98);
        assert_eq!(args.password.len(), 98);
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

        let entity = args.next().unwrap();

        assert_eq!(entity.1.iv.len(), 32);
        assert_eq!(entity.1.key.len(), 32);
        assert_eq!(entity.1.dat.len(), 34);
        assert_eq!(entity.0.as_path().iter().count(), 1);
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

        let entity = args.next().unwrap();

        assert_eq!(entity.1.iv.len(), 32);
        assert_eq!(entity.1.key.len(), 32);
        assert_eq!(entity.1.dat.len(), 34);
        assert_eq!(entity.0.as_path().iter().count(), 1);
        assert_eq!(args.iterator, 1);

        let account = args.next().unwrap();

        assert_eq!(account.1.iv.len(), 32);
        assert_eq!(account.1.key.len(), 32);
        assert_eq!(account.1.dat.len(), 34);
        assert_eq!(account.0.as_path().iter().count(), 2);
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

        let entity = args.next().unwrap();

        assert_eq!(entity.1.iv.len(), 32);
        assert_eq!(entity.1.key.len(), 32);
        assert_eq!(entity.1.dat.len(), 34);
        assert_eq!(entity.0.as_path().iter().count(), 1);
        assert_eq!(args.iterator, 1);

        let account = args.next().unwrap();

        assert_eq!(account.1.iv.len(), 32);
        assert_eq!(account.1.key.len(), 32);
        assert_eq!(account.1.dat.len(), 34);
        assert_eq!(account.0.as_path().iter().count(), 2);
        assert_eq!(args.iterator, 2);

        let password = args.next().unwrap();

        assert_eq!(password.1.iv.len(), 32);
        assert_eq!(password.1.key.len(), 32);
        assert_eq!(password.1.dat.len(), 34);
        assert_eq!(password.0.as_path().iter().count(), 3);
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

        let entity = args.next().unwrap();

        assert_eq!(entity.1.iv.len(), 32);
        assert_eq!(entity.1.key.len(), 32);
        assert_eq!(entity.1.dat.len(), 34);
        assert_eq!(entity.0.as_path().iter().count(), 1);
        assert_eq!(args.iterator, 1);

        let account = args.next().unwrap();

        assert_eq!(account.1.iv.len(), 32);
        assert_eq!(account.1.key.len(), 32);
        assert_eq!(account.1.dat.len(), 34);
        assert_eq!(account.0.as_path().iter().count(), 2);
        assert_eq!(args.iterator, 2);

        let password = args.next().unwrap();

        assert_eq!(password.1.iv.len(), 32);
        assert_eq!(password.1.key.len(), 32);
        assert_eq!(password.1.dat.len(), 34);
        assert_eq!(password.0.as_path().iter().count(), 3);
        assert_eq!(args.iterator, 3);

        let finally_none = args.next();

        assert!(finally_none.is_none());
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

