pub trait PfDatabaseInterface {
    fn set_default_quota(&self, username: &str);
    fn increment_quota(&self, username: &str) -> Result<String, &'static str>;
}

#[derive(Debug, Copy, Clone)]
pub struct PfDatabase {

}

impl PfDatabaseInterface for PfDatabase {
    fn set_default_quota(&self, username: &str) {
        unimplemented!()
    }

    fn increment_quota(&self, username: &str) -> Result<String, &'static str> {
        unimplemented!()
    }
}