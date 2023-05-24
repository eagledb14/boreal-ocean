use chrono::NaiveTime;
use std::net::SocketAddr;
use std::fmt::{Display, Formatter, Result};


#[derive(Debug)]
pub struct Connection {
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

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
