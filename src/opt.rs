use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pfpolicyd", about = "Postfix Policy Checker")]
pub struct Opt {

    /// Local Bind Address ip:port
    #[structopt(short = "l", long = "listen", default_value = "localhost:12345")]
    pub listen: String,

    /// Database Connection URL. Currently only MySQL/MariaDB
    #[structopt(short = "d", long = "dburl", default_value = "mysql://root@localhost:3306/maildata")]
    pub dburl: String,

    /// Sets Timeout for Client Connections
    #[structopt(short = "t", long = "timeout", default_value = "300")]
    pub timeout: u64,

    /// No Action. Accept all messages, but still count. Useful for testing or monitoing only
    #[structopt(short = "n",)]
    pub noaction: bool
}