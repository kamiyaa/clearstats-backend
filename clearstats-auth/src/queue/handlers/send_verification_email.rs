use shared_lib::{
    error::AppServerResult,
    integrations::mailersend::{
        NO_REPLY_EMAIL,
        client::{MailersendClient, MailersendClientTrait},
        types::MailerSendEmailRequestBuilder,
    },
};

use crate::state::AppState;

#[derive(Clone, Debug)]
pub struct Message {
    pub email: String,
    pub verification_code: String,
}

pub async fn handler(app_state: &AppState, message: Message) -> AppServerResult {
    let Message {
        email,
        verification_code,
    } = message;

    let subject = "Email Verification Code".to_string();
    let email_content = generate_invite_email_content(&verification_code);
    let html_email_content = generate_invite_email_html_content(&verification_code);

    let data =
        MailerSendEmailRequestBuilder::new(NO_REPLY_EMAIL.to_string(), "Indaggo".to_string())
            .to_email(email)
            .subject(subject)
            .content(email_content)
            .html_content(html_email_content)
            .build();

    let mailersend_client = MailersendClient::new(app_state.config.mailersend_api_key.clone());
    mailersend_client.send_email(&data).await?;

    Ok(())
}

fn generate_invite_email_content(verification_code: &str) -> String {
    format!("Your verification code is: {verification_code}")
}

fn generate_invite_email_html_content(verification_code: &str) -> String {
    format!("Your verification code is: <pre style=\"font-size: 3rem\">{verification_code}</pre>")
}
