use std::env;

pub struct TelegramClient {
    pub api_key: String
}

impl TelegramClient {
    const URL: &'static str = "https://api.telegram.org/";
    const GET_UPDATE: &'static str = "/getUpdates";
    fn get_url(&self) -> String {
        format!("{}{}{}", TelegramClient::URL, "bot", &self.api_key)
    }
    pub fn send_message(&self) {

    }
    pub fn get_update(&self) {
        let response = reqwest::blocking::get(format!("{}{}", &self.get_url(), TelegramClient::GET_UPDATE)).unwrap().text().unwrap();
        println!("Body:\n {}", response);
    }

    pub fn add_update_listener<F: Fn(String)>(&self, func: F) {
        let response = reqwest::blocking::get(format!("{}{}", &self.get_url(), TelegramClient::GET_UPDATE)).unwrap().text().unwrap();
        func(response)
    }
}
