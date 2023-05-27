use crate::connection::{Connection, GroupedConnection};
use crate::network_parsing::parse_traffic;

use std::{fs, str};
use std::net::IpAddr;
use std::collections::HashMap;
use std::str::FromStr;
use std::thread::JoinHandle;
use pcap::{Capture, Device};
use std::process::{Command, Stdio, Child};
use std::io::{stdin, BufReader, BufRead};
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
        if let Some(new_connection) = parse_traffic(&param) {
            connections.push(new_connection);
        }
    }   

    return connections;
}

pub fn read_tcpdump(device: &str, iterations: Option<String>) -> (Option<JoinHandle<()>>, Receiver<String>) {
    println!("reading");
    // //check if device is valid
    let devices = Device::list().unwrap();
    let mut has_device = false;
    for d in devices {
        if d.name.as_str() == device {
            has_device = true;
        }
    }

    let (sender, receiver) = channel::<String>();
    if !has_device {
        println!("Device not found: {:?}", device);
        return (None, receiver);
    }

    let child = match iterations {
        Some(max_iter) => {
            Command::new("tcpdump")
                    .arg("-n")
                    .arg("-i")
                    .arg(device)
                    .arg("-c")
                    .arg(max_iter)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("Failed on calling TcpDump, make sure you have tcpdump installed")
        }
        None => {
            Command::new("tcpdump")
                    .arg("-n")
                    .arg("-i")
                    .arg(device)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("Failed on calling TcpDump, make sure you have tcpdump installed")
        }
    };

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

pub fn read_tcp_thread(handler: JoinHandle<()>, receiver: Receiver<String>) {


}

pub fn read_tcp_thread_iterations(handler: JoinHandle<()>, receiver: Receiver<String>) -> Vec<Connection> {
    let mut iter_recv = receiver.iter();
    let mut connections_read = Vec::<Connection>::new();

    while let Some(received_string) = iter_recv.next() {
        if let Some(new_connection) = parse_traffic(&received_string) {
            connections_read.push(new_connection);
        }
    }

    handler.join().unwrap();

    return connections_read;
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

// for some reason 0.0.0.0 ip doesn't convert into Grouped connections, for some reason it doesn't
// include the ip address and any destination, even though it is included in the Connection struct
fn get_grouped_connectons(
    connections: &[Connection], 
    old_connections: Vec<GroupedConnection>,
) -> HashMap<IpAddr, GroupedConnection> {
    let mut connection_map: HashMap::<IpAddr, GroupedConnection> = old_connections
                                                                    .into_iter().map(|connection| (connection.source, connection)).collect();

    for connection in connections {
        connection_map.entry(connection.source.ip())
            .and_modify(|c| {
                c.push_destination(connection.destination);
                c.push_port(connection.source.port());
                c.connection_count += 1;
            })
            .or_insert_with(|| GroupedConnection::new(connection.source.ip(), connection.destination, connection.source.port()));

        if connection.source.ip() == IpAddr::from_str("0.0.0.0").unwrap() {
            println!("{:?}", connection.source.port());
            // println!("{:?}\n", connection_map);
        }

    }

    return connection_map;
}
