use std::io;
use std::net::*;

use network::error::Error;
use network::Failable;
use network::messaging::*;

use bitcoin::network::message::{NetworkMessage, RawNetworkMessage};

pub type FailableWithMessages = Result<Messages, Error>;

pub struct Behaviours {
    connection: PeerConnection
}

impl Behaviours {
    pub fn new(addr: SocketAddr) -> Result<Self, io::Error> {
        let connection = PeerConnection::new(addr)?;
        Ok(Self{ connection })
    }

    pub fn run(&mut self) -> Failable {
        let greetings_messages = self.behaviour_greetings()?;
        Ok(())
    }

    pub fn behaviour_greetings(&mut self) -> FailableWithMessages {
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
        self.connection.send_version_message()?;

        let mut buffer = vec![];
        // Let's collect all non-behaviour messages to return them at the end of the function
        let mut messages: Messages = vec![];

        // Now let's loop over all incoming messages untill we collect all messages from the
        // behaviour pattern
        loop {
            let message = RawNetworkMessage::from_stream(&mut self.connection.stream, &mut buffer);
            match message {
                Ok(ref msg) => {
                    println!("Received message: {:?}", msg.payload);
                    match msg.payload {
                        NetworkMessage::Version(_) => {
                            flags.version = true;
                            self.connection.send_verack_message()?;
                        },
                        NetworkMessage::Verack => {
                            flags.verack = true;
                            self.connection.send_addr_message()?
                        },
                        NetworkMessage::Ping(nonce) => {
                            flags.ping = true;
                            self.connection.send_pong_message(nonce)?;
                        },
                        NetworkMessage::Addr(_) => {
                            flags.addr = true;
                            // We need to save this message for a later use to update the list
                            // of known peers
                            messages.push(msg.payload.clone());
                        },
                        NetworkMessage::Alert(_) => {
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
}
