mod pf_policy;

use std::net::TcpStream;
use crate::postfix_client::pf_policy::PfPolicyClient;
use crate::pf_database;
use crate::pf_database::{PfDatabaseInterface, PfDatabase};

pub fn client_handler(client: TcpStream, db_connection: PfDatabase) {

    let mut pf_policy = PfPolicyClient::new(db_connection);
    pf_policy.keepalive(client);

}
