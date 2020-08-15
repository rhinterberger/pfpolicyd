mod pf_policy;

use std::net::TcpStream;
use crate::postfix_client::pf_policy::PfPolicyClient;
use crate::pf_database::{PfDatabase};
use mysql::Pool;

pub fn client_handler(client: TcpStream, db_connection: Pool, noaction: bool) {

    let pf_database = PfDatabase::new(db_connection.get_conn().unwrap());
    PfPolicyClient::keepalive(client, pf_database, noaction);
}
