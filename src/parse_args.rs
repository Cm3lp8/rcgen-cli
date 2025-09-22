use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Args {
    #[arg(long, value_name = "IP_ADDR", num_args=1..,value_parser= clap::value_parser!(IpAddr))]
    ip: Vec<IpAddr>,
}

impl Args {
    pub fn to_strings_vec(&self) -> Vec<String> {
        self.ip.iter().map(|it| it.to_string()).collect()
    }
    pub fn is_emtpy(&self) -> bool {
        self.ip.is_empty()
    }
}
