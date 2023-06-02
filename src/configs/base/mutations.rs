use crate::Base;
use crate::Cipher;

impl Base {
    pub fn mutate<T>(&mut self, adm: T, api: T, web: T)
        where T: ToString
    {
        // Bind base urls
        let admin_url = adm.to_string();
        let api_url = api.to_string();
        let web_url = web.to_string();

        // Set up base urls
        self.admin_url = Some(Cipher::new(admin_url));
        self.api_url = Some(Cipher::new(api_url));
        self.web_url = Some(Cipher::new(web_url));
    }

    pub fn clear(&mut self) {
        self.api_url = None;
        self.web_url = None;
        self.admin_url = None;
    }
}