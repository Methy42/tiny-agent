use std::{
    net::IpAddr,
    process::{self},
    str::FromStr,
};

pub fn check_ip(ip_str: &str) -> IpAddr {
    match IpAddr::from_str(ip_str) {
        Ok(ip_address) => ip_address,
        Err(_) => {
            eprintln!("Invalid server IP address");
            process::exit(1);
        }
    }
}
