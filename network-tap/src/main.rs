use std::{fs, fmt, io};
use chrono::NaiveTime;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use std::env::args;


// TODO
// make it more cli friendly
//// add flags for reading from a file, like it is now
//// add flags to read from stdin
//// add flags to read directly from tcpdump, you have to include the interface and flags, or not
////// give them the flags option
//// for zeek, if you ever figure out how to use that
//// add option to only collect from a certain ip
//// option to group by input and print the destinations
fn main() {

    //if there are not args, then read from stdin
    if args().len() <= 1 {
        let connections = read_stdin();
        print_connections(&connections);
        return;
    }


    for arg in args() {
        println!("{}", arg);
    }

    // let in_string = fs::read_to_string("input-analysis.txt").expect("couldn't find file");
    // let params = in_string.trim().lines();
    // let mut traffic = Vec::<Connection>::new();
    //
    // for param in params {
    //
    //     if let Some(connection) = parse_traffic(&param) {
    //         traffic.push(connection);
    //     }
    // }
    //
    // for t in traffic {
    //     println!("{}\n", t);
    // }

}


fn print_connections(connections: &[Connection]) {
    for connection in connections {
        println!("{}\n", connection);
    }
}

fn read_stdin() -> Vec<Connection> {
    let stdin = io::stdin();
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

fn parse_traffic(params: &str) -> Option<Connection> {
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

    
    let (source, dest) = parse_ip_and_ip6(&parts[2], &parts[4]);

    // grab any additional information from this parameter
    let misc = if params.len() > 5 {
        parts[5..].to_vec()
    }
    else {
        Vec::<String>::new()
    };

    return Some(Connection::new(timestamp, source, dest, misc));
}

fn parse_ip_and_ip6(source_string: &str, destination_string: &str) -> (Option<SocketAddr>, Option<SocketAddr>){
    let source = if let Some(port_location) = source_string.rfind('.') {
        //SocketAddr::from_str only allow for the format ip:port, while the input has the format
        //ip.port and this is the fastest way that I found, possible to benchmark other ways in the
        //future
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

    return (source, destination);
}


#[derive(Debug)]
struct Connection {
    timestamp: NaiveTime,
    source: Option<SocketAddr>,
    destination: Option<SocketAddr>,
    misc: Vec<String>,
}

impl Connection {
    pub fn new(timestamp: NaiveTime, source: Option<SocketAddr>, destination: Option<SocketAddr>, misc: Vec<String>) -> Self {
        Self {
            timestamp,
            source,
            destination,
            misc
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timestamp: {}\n", self.timestamp)?;

        match &self.source {
            Some(source) => write!(f, "Source: {}\n", source)?,
            None => (),
        };

        match &self.destination {
            Some(destination) => write!(f, "Destination: {}\n", destination)?,
            None => (),
        };

        if !(&self.misc.is_empty()) {
            write!(f, "Misc: {}", self.misc.join(", "))
        }
        else {
            write!(f, "")
        }
        
    }
}
