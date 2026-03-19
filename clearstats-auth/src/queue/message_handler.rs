use shared_lib::error::AppServerResult;

use crate::queue::types::ServerMessage;
use crate::state::AppState;

use super::handlers;
use super::types::ServerMessageReceiver;

pub async fn listen(app_state: AppState, mut rx: ServerMessageReceiver) {
    loop {
        if let Some(msg) = rx.recv().await {
            tracing::info!("Message: {:?}", msg);
            let _ = handle_message(&app_state, msg).await;
        }
    }
}

pub async fn handle_message(app_state: &AppState, msg: ServerMessage) -> AppServerResult {
    match msg {
        ServerMessage::SendVerificationEmail {
            email,
            verification_code,
        } => {
            let message = handlers::send_verification_email::Message {
                email,
                verification_code,
            };
            handlers::send_verification_email::handler(app_state, message).await?;
        }
    }
    Ok(())
}
