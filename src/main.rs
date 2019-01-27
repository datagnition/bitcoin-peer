extern crate bitcoin;

use std::net::*;
use std::io::Write;
use bitcoin::network::*;
use bitcoin::consensus::encode;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let seeds : &[SocketAddr] = &[
        SocketAddr::from(([174, 82, 166, 92], 8333)),
        SocketAddr::from(([73, 241, 174, 183], 8333)),
        SocketAddr::from(([37, 187, 0, 47], 8333)),
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


    println!("Bitcoin listener");

    let receiver = &seeds.first().unwrap();
    match TcpStream::connect(receiver) {
        Ok(mut stream) => {
            println!("Connected to the server!");

            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            let version_msg = message::RawNetworkMessage {
                magic: constants::Network::Bitcoin.magic(),
                payload: message::NetworkMessage::Version(message_network::VersionMessage::new(
                    0,
                    since_the_epoch.as_secs() as i64,
                    address::Address::new(receiver, 0),
                    address::Address::new(receiver, 0),
                    0,
                    String::from("macx0r"),
                    0
                ))
            };

            if let Err(err) = stream.write(encode::serialize(&version_msg).as_slice()) {
                eprintln!("Error sending message to the server: {}", err);
            }

            let mut buffer: [u8; 1024] = [0; 1024];

            loop {
                match message::RawNetworkMessage::from_tcpstream(&mut stream, &mut buffer) {
                    Err(err) => eprintln!("Error reading from the server: {}", err),
                    Ok(msg) => {
                        let msg: message::RawNetworkMessage = msg;
                        println!("Received message: {:?}", msg.payload);
                    },
                }
            }

            if let Err(err) = stream.shutdown(Shutdown::Both) {
                eprintln!("Error closing the connection: {}", err);
            }

            ();
        }
        Err(err) => eprintln!("Couldn't connect to server: {}", err),
    }
}
