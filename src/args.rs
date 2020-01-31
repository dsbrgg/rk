use crate::locker::Locker;
use crate::locker::locker_refactor;

#[derive(Clone)]
pub struct Args {
    pub entity: String,
    pub account: String,
    pub password: String
}

impl Args {
    pub fn new(
        entity: Option<&str>,
        account: Option<&str>,
        password: Option<&str>
    ) -> Args {
        let mut locker = locker_refactor::Locker::new();
        
        let mut ent = String::new();
        let mut acc = String::new();
        let mut pwd = String::new();

        if let Some(e) = entity { ent = locker.hash(e); }
        if let Some(a) = account { acc = locker.hash(a); }
        if let Some(p) = password { pwd = locker.encrypt(p); }

        Args {
            entity: ent,
            account: acc,
            password: pwd 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let locker = locker_refactor::Locker::new();
        
        let entity = locker.hash("entity");
        let account = locker.hash("account");

        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        assert_eq!(entity, args.entity);
        assert_eq!(account, args.account);
        // TODO: have to test encryption here
        assert_eq!("password".to_string(), args.password);
    }
}

