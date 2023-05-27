use crate::connection::{Connection, GroupedConnection};
use crate::network_parsing::parse_traffic;

use std::fs;
use std::net::IpAddr;
use std::collections::HashMap;
use std::str::FromStr;
use pcap::{Capture, Device};
use std::process::{Command, Stdio};
use std::str;
use std::io::{stdin, BufReader, BufRead};
use std::time::Instant;

pub fn read_stdin() -> Vec<Connection> {
    // dbg!("reading from stdin");
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
    // dbg!("reading from file");
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

pub fn read_tcpdump(device: &str) -> Vec<Connection> {
    // //check if device is valid
    // let devices = Device::list().unwrap();
    // let mut has_device = false;
    // for d in devices {
    //     if d.name == device {
    //         has_device = false;
    //     }
    // }
    //
    // if !has_device {
    //     println!("Device not found: {:?}", device);
    //     return Vec::new();
    // }

    let child = Command::new("tcpdump")
        .arg("-n")
        .arg("-i")
        .arg("wlan0")
        // .arg("-c 3")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("failure");

    let handle = if let Some(std) = child.stdout {
        println!("reading");
        let reader = BufReader::new(std);
        let hand = std::thread::spawn(move || {
            let now = Instant::now();
            let mut reps = 1;
            println!("{:?}", now.elapsed());

            for line in reader.lines() {
                println!("{}: {:?}", reps, now.elapsed());
                if let Ok(line) = line {
                    // if let Some(line) = parse_traffic(&line) {
                    //     println!("{}: {}\n", reps, line);
                    // }
                }
                reps += 1;
            }
        });

        Some(hand)
    }
    else {
        None
    };

    if let Some(h) = handle {
        _ = h.join();
    }


    Vec::new()
}


pub fn read_binary() {
    let devices = Device::list().unwrap();
    let device = devices[2].clone();

    let mut cap = match Capture::from_device(device).unwrap()
        .promisc(true)
        .snaplen(65535)
        .open() 
        {
            Ok(c) => c,
            Err(e) => {
                dbg!(e);
                return;
            }
    };

    while let Ok(_packet) = cap.next_packet() {
        println!("Packet: {:?}\n",_packet.data);
    }

    drop(cap);
}

pub fn sort_connections(connections: &[Connection]) -> Vec<GroupedConnection> {

    return get_grouped_connectons(connections, vec![]).into_values().collect();
}

pub fn sort_by_ip(connections: &[Connection], ip: String) -> Option<GroupedConnection> {

    if let Some(connection) = get_grouped_connectons(&connections, vec![])
                                .get(&IpAddr::from_str(&ip).ok()?) {
        return Some(connection.clone());
    }
    else {
        return None;
    }
}

fn get_grouped_connectons(
    connections: &[Connection], 
    old_connections: Vec<GroupedConnection>,
) -> HashMap<IpAddr, GroupedConnection> {
            // let mut connection_map = HashMap::<IpAddr, GroupedConnection>::with_capacity(connections.len());
    //
    // for connection in old_connections.into_iter() {
    //     connection_map.insert(connection.source, connection);
    // }
    let mut connection_map: HashMap::<IpAddr, GroupedConnection> = old_connections
                                                                        .into_iter().map(|connection| (connection.source, connection)).collect();

    for connection in connections {
        if let Some(dest) = connection.destination {
            connection_map.entry(connection.source.ip())
                .and_modify(|c| {
                    c.push_destination(dest);
                    c.push_port(connection.source.port());
                })
                .or_insert_with(|| GroupedConnection::new(connection.source.ip()));
        }
    }

    return connection_map;
}
