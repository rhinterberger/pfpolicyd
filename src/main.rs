use std::thread;
use std::net::{TcpListener};
use std::thread::{JoinHandle};
use std::time::Duration;

use mysql;

mod postfix_client;
mod pf_database;
use crate::pf_database::PfDatabase;
use mysql::Pool;

fn main() {

    let mut threads:Vec<JoinHandle<_>> = Vec::new();

    let db_connection_url = "mysql://root:password@localhost:3307/db_name";
    let db_pool = Pool::new(db_connection_url);

    let server = TcpListener::bind("127.0.0.1:1234")
        .expect("Bind failed");

    for client in server.incoming() {
        match client {
            Ok(client_stream) => {
                client_stream.set_read_timeout(Some(Duration::new(10, 0)))
                    .expect("Failed SetReadTimeout");

                threads.push(thread::Builder::new()
                    .name(client_stream
                        .peer_addr()
                        .unwrap()
                        .to_string()
                    )
                    .spawn(move || postfix_client::client_handler(client_stream, PfDatabase {}))
                    .unwrap()
                );
            }
            Err(e) => {
                println!("Client Stream Failed {}", e);
            }
        }
    }

    for thread in threads {
        thread.join().expect("Join Thread Failed");
    }
}
