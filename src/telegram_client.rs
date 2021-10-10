use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::Add;
use std::ptr::null;
use serde::Deserialize;

use native_tls::TlsConnector;
use crate::utils::get_content;

pub struct TelegramClient {
    pub api_key: String,
}

impl TelegramClient {
    const HOST: &'static str = "api.telegram.org";
    const GET_UPDATE: &'static str = "/getUpdates";
    fn get_url(&self) -> String {
        format!("{}{}{}", TelegramClient::HOST, "bot", &self.api_key)
    }
    pub fn send_message(&self) {}

    pub fn add_update_listener<F: Fn(UpdateResponse)>(&self, func: F) {
        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(format!("{}:{}", TelegramClient::HOST, "443")).unwrap();
        let mut stream = connector.connect(TelegramClient::HOST, stream).unwrap();
        let mut request = String::new();
        request.push_str(&format!("GET /bot{}{} HTTP/1.1\r\n", &self.api_key, TelegramClient::GET_UPDATE));
        request.push_str(&format!("Host: {}\r\n", TelegramClient::HOST));
        request.push_str("Connection: keep-alive\r\n");
        request.push_str("Connection-Timeout: 1200\r\n");
        request.push_str("Accept: application/json\r\n\r\n");
        stream.write(request.as_bytes());
        stream.flush();
        let mut body = String::new();
        loop {
            let mut buffer = [0; 1024];
            let result = stream.read(&mut buffer);

            match result {
                Ok(n) => {
                    if n > 0 {
                        body.push_str(&String::from_utf8_lossy(&buffer[..]));
                    }
                }
                _ => println!("None")
            }
            if stream.buffered_read_size().unwrap() == 0 && !body.is_empty() {
                let response = get_content(&body);
                if !response.is_empty() {
                    let update_response:UpdateResponse = serde_json::from_str(response.as_str()).unwrap();
                    func(update_response);
                }
                body.clear();
            }
        }
    }
}
#[derive(Deserialize)]
pub struct UpdateResponse {
    pub ok: bool,
    pub result: Option<Vec<Result>>

}

#[derive(Deserialize)]
pub struct Result {
    pub update_id: i64,
    pub message: Message
}

#[derive(Deserialize)]
pub struct Message {
    pub text: String,
    pub chat: Chat

}

#[derive(Deserialize)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    #[serde(rename="type")]
    pub chat_type: String,
    pub username: String
}
