use chrono::NaiveTime;
use std::net::{SocketAddr, IpAddr};
use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;


#[derive(Debug, Clone)]
pub struct Connection {
    pub timestamp: NaiveTime,
    pub source: SocketAddr,
    pub destination: Option<SocketAddr>,
    pub misc: Vec<String>,
}

impl Connection {
    pub fn new(timestamp: NaiveTime, source: SocketAddr, destination: Option<SocketAddr>, misc: Vec<String>) -> Self {
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

#[derive(Debug, Clone)]
pub struct GroupedConnection {
    pub source: IpAddr,
    pub ports: HashSet<u16>,
    pub destinations: HashSet<SocketAddr>
}

impl GroupedConnection {
    pub fn new(source: IpAddr) -> Self {
        Self {
            source,
            ports: HashSet::new(),
            destinations: HashSet::new()
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
        write!(f, "Destinations: {:?}", self.destinations)
    }
}

