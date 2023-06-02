use crate::Cipher;
use crate::Mailer;

impl Mailer {
    pub fn mutate<T>(
        &mut self,
        username: T,
        password: T,
        smtp_host: T,
        service: T
    )
        where T: ToString
    {
        // Bind mailer config
        let username = username.to_string();
        let password = password.to_string();
        let smtp_host = smtp_host.to_string();
        let service = service.to_string();

        // Set up mailer config
        self.username = Some(Cipher::new(username));
        self.password = Some(Cipher::new(password));
        self.smtp_host = Some(Cipher::new(smtp_host));
        self.service = Some(Cipher::new(service));
    }

    pub fn clear(&mut self) {
        self.username = None;
        self.password = None;
        self.smtp_host = None;
        self.service = None;
    }
}