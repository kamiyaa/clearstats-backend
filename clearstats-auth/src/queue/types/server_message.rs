use tokio::sync::mpsc::{Receiver, Sender};

#[allow(dead_code)]
pub type ServerMessageSender = Sender<ServerMessage>;
pub type ServerMessageReceiver = Receiver<ServerMessage>;

#[derive(Clone, Debug, PartialEq)]
pub enum ServerMessage {
    SendVerificationEmail {
        email: String,
        verification_code: String,
    },
}
