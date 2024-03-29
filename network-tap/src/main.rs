mod network_parsing;
mod file_input;
mod connection;

use clap::Parser;
use file_input::{read_tcp_thread, read_binary, read_stdin, read_file, sort_connections, sort_by_ip, read_tcpdump, read_tcp_thread_iterations};
use connection::Connection;

fn main() {
    let _hram = 2;
    run_cli();
}

fn run_cli() {
    let cli = Cli::parse();

    let mut connections = Vec::<Connection>::new();

    if let Some(files) = cli.files {
        for file in files {
            connections.append(&mut read_file(&file));
        }
    }
    else if let Some(tcp_args) = cli.network_tap {
        let (handler, receiver) = read_tcpdump(&tcp_args, cli.count.clone());

        if let Some(handle) = handler {
            if let Some(_) = cli.count {
                connections.append(&mut read_tcp_thread_iterations(handle, receiver));
            }
            else {
                read_tcp_thread(handle, receiver);
            }
        }
    }
    else if cli.binary {
        read_binary();
    }
    else {
        connections.append(&mut read_stdin());
    }

    //optional sorting
    if cli.sort_by_connection {
        let sorted_connections = sort_connections(&connections);
        for connection in sorted_connections {
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

    #[arg(short= 'c', long = "count", subcommand="network_tap")]
    count: Option<String>,

    #[arg(short = 'b', conflicts_with="network_tap", conflicts_with="files")]
    binary: bool,
}

