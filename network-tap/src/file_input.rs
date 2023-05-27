use crate::connection::{Connection, GroupedConnection};
use crate::network_parsing::parse_traffic;

use std::fs;
use std::net::IpAddr;
use std::collections::HashMap;
use std::str::FromStr;
use std::thread::JoinHandle;
use pcap::{Capture, Device};
use std::process::{Command, Stdio, Child};
use std::str;
use std::io::{stdin, BufReader, BufRead};
use std::time::Instant;

use std::sync::mpsc::{channel, Sender, Receiver};


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


//create a thread that reads from tcpdump
//that writes to a channel or a pipe
//then in another thread, or the maind thread
//I parse the output
//that output is either sorted or printed to stdout
//maybe add an option to read only a certain amount from tcpdump

pub fn read_tcpdump(device: &str) -> (Option<JoinHandle<()>>, Receiver<String>) {
    println!("reading");
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
        .arg(device)
        // .arg("-c 3")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("failure");

    let (sender, receiver) = channel::<String>();
    let handler = spawn_tcpdump_thread(child, sender);

    return (handler, receiver);
}



fn spawn_tcpdump_thread(child: Child, sender: Sender<String>) -> Option<JoinHandle<()>> {
    if let Some(std) = child.stdout {

        let reader = BufReader::new(std);
        let hand = std::thread::spawn(move || {

            for line in reader.lines() {
                if let Ok(line) = line {
                    sender.send(line).unwrap();
                }
            }
        });

        return Some(hand);
    }
    else {
        return None;
    };
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

pub fn append_to_sorted_connections(new_connections: &[Connection], old_connections: Vec<GroupedConnection>) -> Vec<GroupedConnection> {
    return get_grouped_connectons(new_connections, old_connections).into_values().collect();
}

pub fn sort_by_ip(connections: &[Connection], ip: String) -> Option<GroupedConnection> {
    return get_grouped_connectons(&connections, vec![]).get(&IpAddr::from_str(&ip).ok()?).cloned();
}

pub fn append_to_sort_by_ip(connections: &[Connection], ip: String, old_connection: GroupedConnection) -> Option<GroupedConnection> {
    return get_grouped_connectons(&connections, vec![old_connection]).get(&IpAddr::from_str(&ip).ok()?).cloned();
}


fn get_grouped_connectons(
    connections: &[Connection], 
    old_connections: Vec<GroupedConnection>,
) -> HashMap<IpAddr, GroupedConnection> {
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
