#[macro_use]
extern crate aerospike;
extern crate bitcoin;

use std::net::*;

mod db;
mod network;

use db::*;
use network::*;

fn main() {
    let seeds : &[SocketAddr] = &[
        SocketAddr::from(([37, 187, 0, 47], 8333)),
        SocketAddr::from(([73, 241, 174, 183], 8333)),
        SocketAddr::from(([174, 82, 166, 92], 8333)),
        SocketAddr::from(([73, 76, 228, 164], 8333)),
        SocketAddr::from(([172, 104, 244, 173], 8333)),
        SocketAddr::from(([116, 203, 46, 171], 8333)),
        SocketAddr::from(([128, 199, 148, 148], 8333)),
        SocketAddr::from(([138, 68, 1, 45], 8333)),
        SocketAddr::from(([169, 229, 198, 105], 8333)),
        SocketAddr::from(([13, 58, 6, 96], 8333)),
        SocketAddr::from(([72, 130, 216, 43], 8333)),
        SocketAddr::from(([79, 98, 196, 89], 8333)),
        SocketAddr::from(([88, 198, 39, 205], 8333)),
        SocketAddr::from(([104, 248, 80, 132], 8333)),
        SocketAddr::from(([202, 28, 194, 82], 8333)),
        SocketAddr::from(([190, 248, 250, 201], 8333)),
        SocketAddr::from(([71, 222, 111, 159], 8333)),
        SocketAddr::from(([187, 178, 95, 188], 8333)),
        SocketAddr::from(([62, 216, 210, 182], 8333)),
        SocketAddr::from(([173, 254, 210, 229], 8333)),
        SocketAddr::from(([173, 249, 30, 201], 8333)),
        SocketAddr::from(([148, 251, 139, 241], 8333)),
    ];

    let receiver = seeds.first().unwrap();

    println!("Bitcoin listener");

    let mut asp_connector = AspConnector::new();

    let agent = Behaviours::new(*receiver);
    if let Err(err) = agent {
        eprintln!("Couldn't connect to server: {}", err);
        return
    }

    println!("*** Connected to the server");
    if let Err(err) = agent.unwrap().run() {
        eprintln!("Error runnning: {}", err);
    } else {
        println!("*** Finished running");
    }
}
