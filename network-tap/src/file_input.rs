use crate::connection::Connection;
use crate::network_parsing::parse_traffic;

use std::io::stdin;

pub fn read_stdin() -> Vec<Connection> {
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
