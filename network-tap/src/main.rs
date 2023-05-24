use std::fs;
use chrono::{NaiveTime, Local};
use std::net::IpAddr;


fn main() {
    let in_string = fs::read_to_string("tcpdump-test-doc.txt").expect("couldn't find file");
    let params = in_string.trim().lines();

    for param in params {
        let mut parts = param.split(" ").into_iter();

        //get timestamp
        let timestamp = if let Some(time) = parts.next() {
            NaiveTime::parse_from_str(&time, "%T%.f").unwrap()
        }
        else {
            Local::now().time()
        };
        
        // println!("{}", naive_time);


        //get protocal
    
        // let protocal = if let Some(protocol_string) = parts.next() {
        //     switch
        // }
        //
        let protocal = match parts.next() {
            Some("IP") => Protocal::Ip,
            Some("IP6") => Protocal::Ip6,
            Some("ARP,") => Protocal::Arp,
            Some("STP") => Protocal::Stp,
            None => continue,
            Some(e) => {
                // dbg!(e);
                continue;
            },
        };


        // dbg!(protocal);

    }

    // let time = "20:58:26.765637".to_owned();

    // let naive_time = NaiveTime::parse_from_str(&time, "%T%.f").unwrap();



}




struct Connection {
    timestamp: NaiveTime,
    protocal: Protocal,
    source: IpAddr,
    destinatin: IpAddr,
    packet_type: PacketType,
    length: i32
}

#[derive(Debug)]
enum Protocal {
    Ip,
    Ip6,
    Arp,
    Stp,
}

enum Flags {
    Syn,
    Fin,
    Push,
    Rst
}

enum PacketType {
    Tcp{
        flag: Flags,
        sequence: (i32, i32),
        ack: i32,
        win: i32,
        options: Vec<String>,
    },
    Udp,
}



