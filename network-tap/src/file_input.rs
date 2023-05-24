use crate::connection::Connection;
use crate::network_parsing::parse_traffic;

use std::io::stdin;
use std::fs;

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
        dbg!(&param);
        if let Some(new_connection) = parse_traffic(&param) {
            connections.push(new_connection);
        }
    }   

    return connections;
}

