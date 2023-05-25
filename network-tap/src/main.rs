mod network_parsing;
mod file_input;
mod connection;

use clap::Parser;

use file_input::{read_stdin, read_file, sort_connections, sort_by_ip};
use connection::Connection;


// TODO
// make it more cli friendly

//// add flags to read directly from tcpdump, you have to include the interface and flags, or not
////// give them the flags option

//// for zeek, if you ever figure out how to use that
//// add option to only collect from a certain ip
//// option to group by input and print the destinations



// DONE
//// add flags to read from stdin
//// add flags for reading from a file, like it is now

fn main() {

    let cli = Cli::parse();
    // println!("{:?}", cli.files);
    let mut connections = Vec::<Connection>::new();

    if let Some(files) = cli.files {
        for file in files {
            connections.append(&mut read_file(&file));
        }
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

    #[arg(short, long, conflicts_with="tcpdump")]
    files: Option<Vec<String>>,

    #[arg(short = 'x', long = "connection")]
    sort_by_connection: bool,

    #[arg(short = 'i', long = "ip")]
    sort_by_ip: Option<String>,

    #[arg(short, long, conflicts_with="files")]
    tcpdump: Option<String>
}

