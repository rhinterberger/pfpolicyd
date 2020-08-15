use mysql::{PooledConn, Statement, params, Row};
use mysql::prelude::{Queryable};

pub trait PfDatabaseInterface {
    fn set_default_quota(&mut self, username: &str);
    fn increment_quota(&mut self, username: &str);
    fn check_quota_exceeded(&mut self, username: &str) -> Option<String>;
}

#[derive(Debug)]
pub struct PfDatabase {
    connection: PooledConn,
    increment_stmt: Statement,
    quota_exceed_stmt: Statement,
    init_stmt: Statement
}

impl PfDatabaseInterface for PfDatabase {
    fn set_default_quota(&mut self, username: &str) {
        self.connection
            .exec_drop(&self.init_stmt, params!{username})
            .unwrap();
    }

    fn check_quota_exceeded(&mut self, username: &str) -> Option<String> {
        let result:Option<Row> = self.connection
            .exec_first(&self.quota_exceed_stmt, params! {username})
            .unwrap();

        match &result {
            None => { self.set_default_quota(username); },
            Some(row) => {
                if  row["allow_monthly"] == mysql::Value::Int(0) {
                    return Some(String::from("monthly"));
                }
                else if  row["allow_daily"] == mysql::Value::Int(0) {
                    return Some(String::from("daily"));
                }
            }
        }
        None
    }

    fn increment_quota(&mut self, username: &str) {
        eprintln!("Increment {}", username);

        self.connection
            .exec_drop(&self.increment_stmt, params!{username})
            .unwrap();
    }
}

impl PfDatabase {
    pub fn new(mut db_connection: PooledConn) -> PfDatabase {

        let quota_exceed_stmt = db_connection.prep(
            "select current_quotas.daily < default_quotas.daily as allow_daily, current_quotas.monthly < default_quotas.monthly as allow_monthly
            from current_quotas
            left join default_quotas on current_quotas.username = default_quotas.username
            where current_quotas.username = :username"
        ).unwrap();

        let increment_stmt = db_connection.prep(
            "INSERT into current_quotas (username,daily,monthly)
             values (:username,1,1) on duplicate key update daily = daily+1, monthly = monthly+1"
        ).unwrap();

        let init_stmt = db_connection.prep(
            "INSERT IGNORE into default_quotas (username) values (:username);"
        ).unwrap();

        PfDatabase {increment_stmt, connection: db_connection, quota_exceed_stmt, init_stmt }
    }
}
