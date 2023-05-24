mod network_parsing;
mod file_input;
mod connection;

use clap::Parser;

use file_input::{read_stdin, read_file};
use connection::Connection;


// TODO
// make it more cli friendly

//// add flags for reading from a file, like it is now
//// add flags to read directly from tcpdump, you have to include the interface and flags, or not
////// give them the flags option

//// for zeek, if you ever figure out how to use that
//// add option to only collect from a certain ip
//// option to group by input and print the destinations



// DONE
//// add flags to read from stdin

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

    print_connections(&connections);
}

fn print_connections(connections: &[Connection]) {
    for connection in connections {
        println!("{}\n", connection);
    }
}

#[derive(Parser, Debug)]
struct Cli {

    #[arg(short, long, conflicts_with="tcpdump")]
    files: Option<Vec<String>>,

    #[arg(short = 'x', long)]
    sort: bool,

    #[arg(short, long, conflicts_with="files")]
    tcpdump: Option<String>
}

