use chrono::NaiveTime;
use std::net::{SocketAddr, IpAddr};
use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;


#[derive(Debug, Clone)]
pub struct Connection {
    pub timestamp: NaiveTime,
    pub source: SocketAddr,
    pub destination: SocketAddr,
    pub misc: Vec<String>,
}

impl Connection {
    pub fn new(timestamp: NaiveTime, source: SocketAddr, destination: SocketAddr, misc: Vec<String>) -> Self {
        Self {
            timestamp,
            source,
            destination,
            misc
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Timestamp: {}\n", self.timestamp)?;

        write!(f, "Source: {}\n", self.source)?;

        write!(f, "Destination: {}\n", self.destination)?;

        if !(&self.misc.is_empty()) {
            write!(f, "Misc: {}", self.misc.join(", "))
        }
        else {
            write!(f, "")
        }
        
    }
}

#[derive(Debug, Clone)]
pub struct GroupedConnection {
    pub source: IpAddr,
    pub ports: HashSet<u16>,
    pub destinations: HashSet<SocketAddr>,
    pub connection_count: i32,
}

impl GroupedConnection {
    pub fn new(source: IpAddr) -> Self {
        Self {
            source,
            ports: HashSet::new(),
            destinations: HashSet::new(),
            connection_count: 1,
        }
    }

    pub fn push_destination(&mut self, new_destination: SocketAddr) {
        self.destinations.insert(new_destination);
    }

    pub fn push_port(&mut self, new_port: u16) {
        self.ports.insert(new_port);
    }
}

impl Display for GroupedConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Source: {}\n", self.source)?;
        write!(f, "Ports: {:?}\n", self.ports)?;
        write!(f, "Destinations: {:?}\n", self.destinations)?;
        write!(f, "Connections: {}", self.connection_count)
    }
}

