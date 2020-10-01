use std::path::PathBuf;
use crate::locker::{Locker, Encrypted};

/* Args struct */

#[derive(Clone, Debug)]
pub struct Args {
    iterator: u8,
    pub entity: Encrypted,
    pub account: Encrypted,
    pub password: Encrypted,
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
        if self.has_password() { path.push(self.password.path()); } 

        path 
    } 

    pub fn has_entity(&self) -> bool {
        !self.entity.is_empty()
    }

    pub fn has_account(&self) -> bool {
        !self.account.is_empty()
    }

    pub fn has_password(&self) -> bool {
        !self.password.is_empty()
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

    #[test]
    fn has_password() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        let has_password = args.has_password();

        assert_eq!(has_password, true);
    }

    #[test]
    fn no_password() {
        let args = Args::new(
            Some("entity"),
            Some("account"),
            None
        );

        let has_password = args.has_password();

        assert_eq!(has_password, false);
    }
}
