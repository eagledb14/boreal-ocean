mod network_parsing;
mod file_input;
mod connection;

use std::env::args;


use file_input::*;
use connection::Connection;


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
}

fn print_connections(connections: &[Connection]) {
    for connection in connections {
        println!("{}\n", connection);
    }
}
