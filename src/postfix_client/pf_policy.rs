use std::collections::HashMap;
use std::io::{BufReader, BufRead, Write};
use std::net::{TcpStream, Shutdown};
use crate::pf_database::{PfDatabaseInterface, PfDatabase};


#[derive(Debug, Copy, Clone)]
pub struct PfPolicyClient {
}

impl PfPolicyClient {

    pub fn keepalive(client_stream: TcpStream, mut database: PfDatabase, noaction: bool) {
        eprintln!("{} : New Connection", client_stream.peer_addr().unwrap());

        let mut client = BufReader::new(&client_stream);
        let mut request: HashMap<String, String> = HashMap::new();
        let pf_client = PfPolicyClient {};

        loop {
            match pf_client.read_request_line(&mut client) {
                Ok(Some(line)) => { request.insert(line[0].clone(), line[1].clone()); },
                Ok(None) => {
                    let mut response = pf_client.handle_request(&request, &mut database);
                    if noaction {
                        response=String::from("action=defer_if_reject\n\n");
                    }
                    client
                        .get_mut()
                        .write(response.as_bytes())
                        .expect("Write Failed");
                },
                Err(error) => {
                    eprintln!("{} : Connection closed: {}", client_stream.peer_addr().unwrap(), error);
                    client_stream.shutdown(Shutdown::Both).expect("Shutdown failed");
                    break;
                }
            };
        }
    }

    pub fn read_request_line(self, client_reader: &mut BufReader<&TcpStream>) -> Result<Option<Vec<String>>, &'static str> {
        let mut request_line = String::new();

        match client_reader.read_line(&mut request_line) {
            Ok(0) | Err(_) => { return Err("Client Disconnect"); }
            Ok(_) => ()
        }

        let line_key_value = self.get_key_value(request_line.clone())?;
        Ok(line_key_value.clone())
    }

    fn get_key_value(self, line: String) -> Result<Option<Vec<String>>, &'static str> {
        let split_line: Vec<String> = line
            .split('=')
            .map(|part| String::from(part.trim()))
            .collect();

        if split_line.len() == 2 {
            return Ok(Some(split_line.clone()));
        }
        if split_line[0] == "" {
            return Ok(None);
        }
        Err("Invalid Line")
    }

    pub fn handle_request(self, request: &HashMap<String, String>, database: &mut PfDatabase) -> String {
        eprintln!("{:?}", request);
        match database.check_quota_exceeded(request.get("sasl_username").unwrap()) {
            None => { database.increment_quota(request.get("sasl_username").unwrap()); }
            Some(period) => {
                database.increment_quota(request.get("sasl_username").unwrap());
                return format!("action=defer_if_permit Service temporarily unavailable - {} send quota exceeded\n\n", period);
            }
        }
        String::from("action=defer_if_reject\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_key_value() {
        let pf_client = PfPolicyClient {};
        let actual = pf_client.get_key_value(String::from("reverse_client_name=another.domain.tld")).unwrap();
        assert_eq!(actual, Some(vec!(String::from("reverse_client_name"),String::from("another.domain.tld"))));

        let actual = pf_client.get_key_value(String::from("Invalid Line"));
        assert_eq!(actual, Err("Invalid Line"));

        let actual = pf_client.get_key_value(String::from("\n"));
        assert_eq!(actual, Ok(None));
    }
}