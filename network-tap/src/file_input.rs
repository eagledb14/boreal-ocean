use crate::connection::{Connection, GroupedConnection};
use crate::network_parsing::parse_traffic;

use std::io::stdin;
use std::fs;
use std::net::{SocketAddr, IpAddr};
use std::collections::HashMap;
use std::str::FromStr;

pub fn read_stdin() -> Vec<Connection> {
    dbg!("reading from stdin");
    let stdin = stdin();
    let mut connections = Vec::<Connection>::new();

    for line in stdin.lines() {
        if let Ok(param) = line {
            if let Some(new_connection) = parse_traffic(&param){
                connections.push(new_connection);
            }
        }
    }

    return connections;
}


pub fn read_file(file_string: &str) -> Vec<Connection> {
    dbg!("reading from file");
    let file = match fs::read_to_string(file_string) {
        Ok(file) => file,
        Err(e) => {
            dbg!(e);
            return Vec::new();
        },
    };
    let mut connections = Vec::<Connection>::new();

    for param in file.lines() {
        // dbg!(&param);
        if let Some(new_connection) = parse_traffic(&param) {
            connections.push(new_connection);
        }
    }   

    return connections;
}


pub fn sort_connections(connections: &[Connection]) -> Vec<GroupedConnection> {

    return get_grouped_connectons(connections).into_values().collect();
}



pub fn sort_by_ip(connections: &[Connection], ip: String) -> Option<GroupedConnection> {

    if let Some(connection) = get_grouped_connectons(&connections)
                                .get(&IpAddr::from_str(&ip).ok()?) {
        return Some(connection.clone());
    }
    else {
        return None;
    }

}


fn get_grouped_connectons(connections: &[Connection]) -> HashMap<IpAddr, GroupedConnection> {
    let mut connection_map = HashMap::<IpAddr, GroupedConnection>::with_capacity(connections.len());

    for connection in connections {
        if let Some(dest) = connection.destination {
            connection_map.entry(connection.source.ip())
                .and_modify(|c| {
                    c.push_destination(dest);
                    c.push_port(connection.source.port());
                })
                .or_insert(GroupedConnection::new(connection.source.ip()));
        }
    }

    return connection_map;
}
