use crate::locker::Locker;

#[derive(Clone, Debug)]
pub struct Args {
    pub entity: String,
    pub account: String,
    pub password: String
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

        if let Some(e) = entity { ent = locker.hash(e); }
        if let Some(a) = account { acc = locker.hash(a); }
        if let Some(p) = password { pwd = locker.encrypt(p); }

        Args {
            entity: ent,
            account: acc,
            password: pwd 
        }
    }

    /* Methods */

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
        
        let entity = locker.hash("entity");
        let account = locker.hash("account");

        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        assert_eq!(entity, args.entity);
        assert_eq!(account, args.account);
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

