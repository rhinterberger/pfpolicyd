use std::{thread};
use std::net::{TcpListener, TcpStream};
use std::thread::{JoinHandle};
use std::time::Duration;

use mysql;
use mysql::Pool;
use structopt::StructOpt;

mod opt;
mod postfix_client;
mod pf_database;

fn main() {
    let mut threads:Vec<JoinHandle<_>> = Vec::new();

    let params = opt::Opt::from_args();
    let timeout = params.timeout;
    let noaction = params.noaction;

    let db_pool = Pool::new(params.dburl)
        .expect("Database Connection failed");

    let server = TcpListener::bind(params.listen)
        .expect("Bind to IP:Port failed");

    for client in server.incoming() {
        match client {
            Ok(mut client_stream) => {
                let db_connection = db_pool.clone();
                let new_thread= prepare_thread(&mut client_stream, timeout)
                    .spawn(move || postfix_client::client_handler(client_stream,  db_connection, noaction));

                match new_thread {
                    Ok(handle) => {
                        threads.push(handle);
                    }
                    Err(error) => {
                        eprintln!("Start Thread Failed {}", error);
                    }
                }
            }
            Err(error) => {
                eprintln!("Client Stream Failed {}", error);
            }
        }
    }

    for thread in threads {
        thread.join()
            .expect("Join Thread Failed");
    }
}

fn prepare_thread(client: &mut TcpStream, timeout: u64) -> thread::Builder {
    client.set_read_timeout(Some(Duration::new(timeout, 0)))
        .expect("Failed SetReadTimeout");

    let client_address = client
        .peer_addr()
        .unwrap()
        .to_string();

    thread::Builder::new().name(client_address)
}