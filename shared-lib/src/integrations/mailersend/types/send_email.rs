use std::collections::HashMap;

use serde::Serialize;

use crate::integrations::mailersend::client::email;

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct SendEmailFrom {
    pub email: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct SendEmailTo {
    pub email: String,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct SendEmailVariable {
    pub email: String,
    pub substitutions: Vec<SendEmailVar>,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct SendEmailVar {
    pub var: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct MailerSendEmailRequestBuilder {
    request: email::send_email::RequestBody,
}

impl MailerSendEmailRequestBuilder {
    pub fn new(email: String, name: String) -> Self {
        let mut request = email::send_email::RequestBody::default();
        request.from.email = email;
        request.from.name = name;
        Self { request }
    }

    pub fn from(mut self, email: String, name: String) -> Self {
        self.request.from.email = email;
        self.request.from.name = name;
        self
    }
    pub fn to_email(mut self, email: String) -> Self {
        self.request.to.push(SendEmailTo { email });
        self
    }
    pub fn subject(mut self, subject: String) -> Self {
        self.request.subject = Some(subject);
        self
    }
    pub fn content(mut self, content: String) -> Self {
        self.request.text = content;
        self
    }
    pub fn html_content(mut self, html_content: String) -> Self {
        self.request.html = html_content;
        self
    }
    pub fn variable(mut self, email: String, var: HashMap<String, String>) -> Self {
        self.request.variables.push(SendEmailVariable {
            email,
            substitutions: var
                .into_iter()
                .map(|(k, v)| SendEmailVar { var: k, value: v })
                .collect(),
        });
        self
    }
    pub fn build(self) -> email::send_email::RequestBody {
        self.request
    }
}
