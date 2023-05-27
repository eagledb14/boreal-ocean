mod network_parsing;
mod file_input;
mod connection;

use clap::Parser;

use file_input::{read_binary, read_stdin, read_file, sort_connections, sort_by_ip, read_tcpdump};
use connection::Connection;


// TODO
// make it more cli friendly

//// add flags to read directly from tcpdump, you have to include the interface and flags, or not

//// for zeek, if you ever figure out how to use that



// DONE
//// add flags to read from stdin
//// add flags for reading from a file, like it is now
//// add option to only collect from a certain ip
//// option to group by input and print the destinations

fn main() {
    run_cli();
}

fn run_cli() {
    let cli = Cli::parse();
    // println!("{:?}", cli.files);
    let mut connections = Vec::<Connection>::new();

    if let Some(files) = cli.files {
        for file in files {
            connections.append(&mut read_file(&file));
        }
    }
    else if let Some(tcp_args) = cli.network_tap {
        read_tcpdump(&tcp_args);
    }
    else if cli.binary {
        read_binary();
    }
    else {
        connections.append(&mut read_stdin());
    }

    //optional sorting
    if cli.sort_by_connection {
        let connections = sort_connections(&connections);
        for connection in connections {
            println!("{}\n", connection);
        }
    }
    else if let Some(ip) = cli.sort_by_ip {
        if let Some(connection) = sort_by_ip(&connections, ip) {
            println!("{}", connection);
        }   
        else {
            println!("Ip not in the dataset", );
        }
    }
    else {
        for connection in connections {
            println!("{}\n", connection);
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {

    #[arg(short, long, conflicts_with="network_tap", conflicts_with="binary")]
    files: Option<Vec<String>>,

    #[arg(short = 'x', long = "connection")]
    sort_by_connection: bool,

    #[arg(short = 'i', long = "ip")]
    sort_by_ip: Option<String>,

    #[arg(short = 't', long = "tap", conflicts_with="files", conflicts_with="binary")]
    network_tap: Option<String>,

    #[arg(short = 'b', conflicts_with="network_tap", conflicts_with="files")]
    binary: bool,
}

