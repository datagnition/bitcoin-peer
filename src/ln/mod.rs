

use lightning::util::logger::{Logger, Record};
use lightning::ln::peer_handler::{MessageHandler, PeerManager};
use lightning::ln::msgs::{ChannelMessageHandler, RoutingMessageHandler};

struct SimpleLogger { }
impl Logger for SimpleLogger {
    fn log(&self, record: &Record) {
        println!("{?:}", record);
    }
}

