use crate::locker::{Locker, Encrypted};

/* Args struct */

#[derive(Clone, Debug)]
pub struct Args {
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
        let args = Args::new(
            Some("entity"),
            Some("account"),
            Some("password")
        );

        // 0x<encrypted = 34>$0x<encrypted = 34>$0x<encrypted = 34>$<hash = 64>
        assert_eq!(args.entity.path().len(), 169);
        assert_eq!(args.account.path().len(), 169);
        assert_eq!(args.password.path().len(), 169);
    }
}
