use std::net::*;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoin::network::{message, message_network, address, constants};
use bitcoin::consensus::encode;

use network::Failable;

pub type Messages = Vec<message::NetworkMessage>;

macro_rules! encode {
    ( $payload:expr ) => {
        &encode::serialize(&message::RawNetworkMessage {
            magic: constants::Network::Bitcoin.magic(),
            payload: $payload,
        }).as_slice()
    };
}

pub struct PeerConnection {
    pub stream: TcpStream,
    pub addr: SocketAddr,
}

impl PeerConnection {
    pub fn new(addr: SocketAddr) -> Result<Self, io::Error> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream, addr })
    }

    pub fn send_version_message(&mut self) -> Failable {
        let msg = message::NetworkMessage::Version(
            message_network::VersionMessage::new(
                0,
                Self::get_current_timestamp() as i64,
                address::Address::new(&self.addr, 0),
                address::Address::new(&self.addr, 0),
                0,
                String::from("macx0r"),
                0
            )
        );
        self.stream.write(encode!(msg))?;
        println!("Version message sent");
        Ok(())
    }

    pub fn send_verack_message(&mut self) -> Failable {
        self.stream.write(encode!(message::NetworkMessage::Verack))?;
        println!("Verack message sent");
        Ok(())
    }

    pub fn send_addr_message(&mut self) -> Failable {
        let msg = message::NetworkMessage::Addr(vec![
            (Self::get_current_timestamp(), address::Address::new(&self.addr, 0))
        ]);
        self.stream.write(encode!(msg))?;
        println!("Addr message sent");
        Ok(())
    }

    pub fn send_pong_message(&mut self, nonce: u64) -> Failable {
        self.stream.write(encode!(message::NetworkMessage::Pong(nonce)))?;
        println!("Pong message sent");
        Ok(())
    }

    fn get_current_timestamp() -> u32 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs() as u32
    }
}

impl Drop for PeerConnection {
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}
