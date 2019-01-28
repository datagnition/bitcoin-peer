extern crate bitcoin;

use std::net::*;
use std::io::{self, Write};
use std::fmt;
use bitcoin::network::{message, message_network, address, constants};
use bitcoin::consensus::encode;
use std::time::{SystemTime, UNIX_EPOCH};
use std::error;

fn main() {
    println!("Bitcoin listener");

    if let Err(err) = run() {
        eprintln!("Couldn't connect to server: {}", err);
    }
}

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    //NetworkError(network::Error),
    DataError(encode::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref e) => fmt::Display::fmt(e, f),
            //Error::NetworkError(ref e) => fmt::Display::fmt(e, f),
            Error::DataError(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => e.description(),
            //Error::NetworkError(ref e) => e.description(),
            Error::DataError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref e) => Some(e),
            //Error::NetworkError(ref e) => Some(e),
            Error::DataError(ref e) => Some(e),
        }
    }
}

#[doc(hidden)]
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}


fn run() -> Result<(), Error> {
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
    let mut stream = TcpStream::connect(receiver)?;
    println!("Connected to the server");

    send_version_message(&mut stream, receiver);

    let mut buffer = vec![];
    loop {
        let message = message::RawNetworkMessage::from_stream(&mut stream, &mut buffer);
        match message {
            Ok(ref msg) => {
                println!("Received message: {:?}", msg.payload);
                match msg.payload {
                    message::NetworkMessage::Verack => {
                        send_verack_message(&mut stream);
                        send_addr_message(&mut stream, &receiver)
                    },
                    message::NetworkMessage::Ping(nonce) =>
                        send_pong_message(&mut stream, nonce),
                    _ => continue,
                }
            },
            Err(err) => {
                stream.shutdown(Shutdown::Both)?;
                return Err(Error::DataError(err))
            },
        };
    }
}

macro_rules! encode {
    ( $payload:expr ) => {
        &encode::serialize(&message::RawNetworkMessage {
            magic: constants::Network::Bitcoin.magic(),
            payload: $payload,
        }).as_slice()
    };
}

fn get_current_timestamp() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

fn send_version_message(stream: &mut TcpStream, addr: &SocketAddr) -> Result<(), io::Error> {

    let msg = message::NetworkMessage::Version(
        message_network::VersionMessage::new(
            0,
            get_current_timestamp() as i64,
            address::Address::new(addr, 0),
            address::Address::new(addr, 0),
            0,
            String::from("macx0r"),
            0
        )
    );
    stream.write(encode!(msg));
    println!("Version message sent");
    Ok(())
}

fn send_verack_message(stream: &mut TcpStream) -> Result<(), io::Error> {
    stream.write(encode!(message::NetworkMessage::Verack))?;
    println!("Verack message sent");
    Ok(())
}

fn send_addr_message(stream: &mut TcpStream, addr: &SocketAddr) -> Result<(), io::Error> {
    let msg = message::NetworkMessage::Addr(vec![
        (get_current_timestamp(), address::Address::new(&addr, 0))
    ]);
    stream.write(encode!(msg));
    println!("Addr message sent");
    Ok(())
}

fn send_pong_message(stream: &mut TcpStream, nonce: u64) -> Result<(), io::Error> {
    stream.write(encode!(message::NetworkMessage::Pong(nonce)))?;
    println!("Pong message sent");
    Ok(())
}
