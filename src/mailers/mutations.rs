use crate::Mailer;

impl Mailer {
    pub fn mutate(&mut self, form: &Self) {
        self.username = form.username.clone();
        self.password = form.password.clone();
        self.smtp_host = form.smtp_host.clone();
        self.service = form.service.clone();
    }

    pub fn clear(&mut self) {
        self.username = None;
        self.password = None;
        self.smtp_host = None;
        self.service = None;
    }
}