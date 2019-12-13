pub struct Args<'a> {
    entity: &'a str,
    account: &'a str,
    password: &'a str
}

impl<'a> Args<'a> {
    fn new(
        entity: Option<&str>,
        account: Option<&str>,
        password: Option<&str>
    ) -> Args<'a> {

    }
}
