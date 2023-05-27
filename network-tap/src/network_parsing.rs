use crate::connection::Connection;
use chrono::NaiveTime;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
//use std::process::Command;

pub fn parse_traffic(params: &str) -> Option<Connection> {
    let parts = params.split(" ").map(String::from).collect::<Vec<String>>();

    //continues if the string input is not a part of the ip or ip6 protocal
    if let Some(protocal) = parts.get(1) {
        match protocal.as_str() {
            "IP" | "IP6" => (),
            _ => return None,
        };
    }

    let timestamp = match NaiveTime::parse_from_str(&parts[0], "%T%.f") {
        Ok(time) => time,
        Err(_) => return None,
    };

    
    let source = match parse_source(&parts[2]) {
        Some(s) => s,
        None => return None,
    };


    let dest = match parse_destination(&parts[4]) {
        Some(d) => d,
        None => return None,
    };


    // grab any additional information from this parameter
    let misc = if params.len() > 5 {
        parts[5..].to_vec()
    }
    else {
        Vec::<String>::new()
    };

    return Some(Connection::new(timestamp, source, dest, misc));
}

fn parse_source(source_string: &str) -> Option<SocketAddr> {
    //SocketAddr::from_str only allow for the format ip:port, while the input has the format
    //ip.port and this is the fastest way that I found, possible to benchmark other ways in the
    //future
    let source = if let Some(port_location) = source_string.rfind('.') {
        let str_addr = format!("{}:{}", &source_string[..port_location], &source_string[(port_location + 1)..]);
        SocketAddr::from_str(&str_addr).ok()
    }
    else {

        let sock_addr = match IpAddr::from_str(&source_string) {
            Ok(str_addr) => Some(SocketAddr::new(str_addr, 0)),
            Err(_) => None,
        };
            
        sock_addr
    };

    return source;
}

fn parse_destination(destination_string: &str) -> Option<SocketAddr>{
    let destination_string = destination_string.trim_end_matches(":");
    let destination = if let Some(port_location) = destination_string.rfind('.') {
        let str_addr = format!("{}:{}", &destination_string[..port_location], &destination_string[(port_location + 1)..]);
        SocketAddr::from_str(&str_addr).ok()
    }
    else {
        let sock_addr = match IpAddr::from_str(&destination_string) {
            Ok(str_addr) => Some(SocketAddr::new(str_addr, 0)),
            Err(_) => None,
        };

        sock_addr
    };

    return destination;
}
