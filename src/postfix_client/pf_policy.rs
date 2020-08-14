use std::collections::HashMap;
use std::io::{BufReader, BufRead, Write};
use std::net::{TcpStream, Shutdown};
use crate::pf_database::{PfDatabaseInterface, PfDatabase};


#[derive(Debug, Copy, Clone)]
pub struct PfPolicyClient {
    database: PfDatabase
}

impl PfPolicyClient {

    pub fn new(database: PfDatabase) -> Box<PfPolicyClient> {

        let pf = PfPolicyClient {database};
        Box::new(pf)
    }

    pub fn keepalive(&mut self, client_stream: TcpStream) {

        println!("{} : New Connection", client_stream.peer_addr().unwrap());

        let mut client = BufReader::new(&client_stream);

        let mut request: HashMap<String, String> = HashMap::new();
        loop {

            match self.read_request_line(&mut client) {
                Ok(Some(line)) => { request.insert(line[0].clone(), line[1].clone()); },
                Ok(None) => { self.handle_request(&request, &mut client); },
                Err(error) => {
                    println!("{} : Connection closed: {}", client_stream.peer_addr().unwrap(), error);
                    client_stream.shutdown(Shutdown::Both);
                    break;
                }
            };
        }
    }

    pub fn read_request_line(self, client_reader: &mut BufReader<&TcpStream>) -> Result<Option<Vec<String>>, &'static str> {
        let mut request_line = String::new();

        match client_reader.read_line(&mut request_line) {
            Ok(0) | Err(_) => {return Err("Client Disconnect");}
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

    pub fn handle_request(self, request: &HashMap<String, String>, client: &mut BufReader<&TcpStream>) -> String {
      //  println!("{:?}", request);
        client
            .get_mut()
            .write(b"lalal")
            .expect("Write Failed");
        String::from("Lalala\n")
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_key_value() {
        let actual = get_key_value(String::from("reverse_client_name=another.domain.tld")).unwrap();
        assert_eq!(actual, Some(vec!(String::from("reverse_client_name"),String::from("another.domain.tld"))));

        let actual = get_key_value(String::from("Invalid Line"));
        assert_eq!(actual, Err("Invalid Line"));

        let actual = get_key_value(String::from("\n"));
        assert_eq!(actual, Ok(None));
    }
    /*
        #[test]
        fn test_handle_request() {
            let actual = parse_request(test_request);
            let expected: HashMap<&str, &str> = [
                ("request", "smtpd_access_policy"),
                ("protocol_state", "RCPT"),
                ("protocol_name", "SMTP"),
                ("helo_name", "some.domain.tld"),
                ("queue_id", "8045F2AB23"),
                ("sender", "foo@bar.tld"),
                ("recipient", "bar@foo.tld"),
                ("recipient_count", "0"),
                ("client_address", "1.2.3.4"),
                ("client_name", "another.domain.tld"),
                ("reverse_client_name", "another.domain.tld"),
                ("instance", "123.456.7"),
                ("sasl_method", "plain"),
                ("sasl_username", "you"),
                ("sasl_sender", ""),
                ("size", "12345"),
                ("ccert_subject", "solaris9.porcupine.org"),
                ("ccert_issuer", "Wietse+20Venema"),
                ("ccert_fingerprint", "C2:9D:F4:87:71:73:73:D9:18:E7:C2:F3:C1:DA:6E:04"),
                ("encryption_protocol", "TLSv1/SSLv3"),
                ("encryption_cipher", "DHE-RSA-AES256-SHA"),
                ("encryption_keysize", "256"),
                ("etrn_domain", ""),
                ("stress", ""),
                ("ccert_pubkey_fingerprint", "68:B3:29:DA:98:93:E3:40:99:C7:D8:AD:5C:B9:C9:40"),
                ("client_port", "1234"),
                ("policy_context", "submission"),
                ("server_address", "10.3.2.1"),
                ("server_port", "54321")
            ].iter().cloned().collect();
            assert_eq!(actual, expected);
        }

     */
}*/