use actix_web::Result;
use infer::Infer;
use lettre::{ Message, SmtpTransport, Transport };
use lettre::message::{ header::ContentType, Attachment, MultiPart, SinglePart };
use lettre::transport::smtp::authentication::Credentials;

use crate::Mailer;
use crate::MailerAttachment;
use crate::Payload;

use crate::traits::Decrypt;

/// Mailer implementation
impl Mailer {
    /// Semd email via SMTP
    pub async fn send<F, T, S, B>(&self, from: F, to: T, subject: S, body: B, attachments: &[MailerAttachment]) -> Result<String>
        where F: ToString,
              T: ToString,
              S: ToString,
              B: ToString
    {
        // Make sure values were properly decrypted
        let data = match self.clone().decrypt() {
            None => self.clone(),
            Some(data) => data
        };

        // Set bindings
        let from = from.to_string();
        let to = to.to_string();
        let subject = subject.to_string();
        let body = body.to_string();

        let username = data.username.clone().unwrap_or_default().get_string_from_master();
        let password = data.password.clone().unwrap_or_default().get_string_from_master();
        let smtp_host = data.smtp_host.unwrap_or_default().get_string_from_master();

        // Create multipart body
        let mut multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body)
            );

        // Check if file exists
        for attachment in attachments {
            let filename = attachment.filename.clone();
            let name = attachment.name.clone();

            match std::fs::read(&filename) {
                Ok(file) => {
                    let info = Infer::new();
                    match ContentType::parse(&info
                        .get(&file.clone())
                        .map_or(String::default(), |t| String::from(t.mime_type()))) {
                        Ok(content_type) => {
                            multipart = multipart.singlepart(
                                Attachment::new(name).body(file, content_type)
                            );
                        },
                        Err(_) => continue
                    };
                },
                Err(_) => continue
            }
        }

        // Create email builder
        let builder = match Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(multipart) {
            Ok(builder) => builder,
            Err(error) => return Err(Payload::error(&error))
        };

        // Set credentials
        let credentials = Credentials::new(username, password);

        // Set smtp transport relay
        let relay = match SmtpTransport::relay(smtp_host.as_str()) {
            Ok(relay) => relay,
            Err(error) => return Err(Payload::error(&error))
        };

        // Open a remote connection
        let mailer = relay.credentials(credentials).build();

        // Send the email
        match mailer.send(&builder) {
            Ok(_) => Ok(format!("Email sent successfully to {to}")),
            Err(error) => Err(Payload::error(&error)),
        }
    }
}