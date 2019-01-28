extern crate bitcoin;

use std::net::*;
use std::io::{self, Write};
use std::fmt;
use bitcoin::network::{message, message_network, address, constants};
use bitcoin::consensus::encode;
use std::time::{SystemTime, UNIX_EPOCH};
use std::error;

type Failable = Result<(), Error>;
type Messages = Vec<message::NetworkMessage>;
type FailableWithMessages = Result<Messages, Error>;

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


fn run() -> Failable {
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
    println!("*** Connected to the server");

    let greetings_messages = behaviour_greetings(&mut stream, &receiver)?;
    println!("*** Greetings behaviour completed successfully");

    Ok(())
}

fn behaviour_greetings(stream: &mut TcpStream, addr: &SocketAddr) -> FailableWithMessages {
    // Flags to determine passing through the behaviour states
    #[derive(Default)]
    struct Flags {
        verack: bool,
        version: bool,
        addr: bool,
        ping: bool
    }
    let mut flags: Flags = Default::default();

    // Behavior starts with the first Version message sent from our side
    send_version_message(stream, addr)?;

    let mut buffer = vec![];
    // Let's collect all non-behaviour messages to return them at the end of the function
    let mut messages: Messages = vec![];

    // Now let's loop over all incoming messages untill we collect all messages from the
    // behaviour pattern
    loop {
        let message = message::RawNetworkMessage::from_stream(stream, &mut buffer);
        match message {
            Ok(ref msg) => {
                println!("Received message: {:?}", msg.payload);
                match msg.payload {
                    message::NetworkMessage::Version(_) => {
                        flags.version = true;
                        send_verack_message(stream)?;
                    },
                    message::NetworkMessage::Verack => {
                        flags.verack = true;
                        send_addr_message(stream, &addr)?
                    },
                    message::NetworkMessage::Ping(nonce) => {
                        flags.ping = true;
                        send_pong_message(stream, nonce)?;
                    },
                    message::NetworkMessage::Addr(_) => {
                        flags.addr = true;
                        // We need to save this message for a later use to update the list
                        // of known peers
                        messages.push(msg.payload.clone());
                    },
                    message::NetworkMessage::Alert(_) => {
                        // This is old-behaving agent: alert message is depricated. Just ignore it.
                    },
                    _ => {
                        // Normally greeting part should consist only of Version, Verack, Ping/Pong,
                        // and Addr messages; so we need to inform that the greeting went wrong
                        // if we received any other message
                        messages.push(msg.payload.clone());
                    },
                }
            },
            Err(err) => {
                stream.shutdown(Shutdown::Both)?;
                return Err(Error::DataError(err))
            },
        };

        // Now we have run the whole greetings protocol and can return with all collected
        // non-greeting message set
        if flags.version && flags.verack && flags.addr && flags.ping {
            return Ok(messages);
        }
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

fn send_version_message(stream: &mut TcpStream, addr: &SocketAddr) -> Failable {

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
    stream.write(encode!(msg))?;
    println!("Version message sent");
    Ok(())
}

fn send_verack_message(stream: &mut TcpStream) -> Failable {
    stream.write(encode!(message::NetworkMessage::Verack))?;
    println!("Verack message sent");
    Ok(())
}

fn send_addr_message(stream: &mut TcpStream, addr: &SocketAddr) -> Failable {
    let msg = message::NetworkMessage::Addr(vec![
        (get_current_timestamp(), address::Address::new(&addr, 0))
    ]);
    stream.write(encode!(msg))?;
    println!("Addr message sent");
    Ok(())
}

fn send_pong_message(stream: &mut TcpStream, nonce: u64) -> Failable {
    stream.write(encode!(message::NetworkMessage::Pong(nonce)))?;
    println!("Pong message sent");
    Ok(())
}
