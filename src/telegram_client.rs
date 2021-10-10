use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::Add;
use native_tls::TlsConnector;

pub struct TelegramClient {
    pub api_key: String
}

impl TelegramClient {
    const TCP_ADDR: &'static str = "api.telegram.org:443";
    const HOST: &'static str = "api.telegram.org";
    const GET_UPDATE: &'static str = "/getUpdates";
    fn get_url(&self) -> String {
        format!("{}{}{}", TelegramClient::HOST, "bot", &self.api_key)
    }
    pub fn send_message(&self) {

    }

    pub fn add_update_listener<F: Fn(String)>(&self, func: F) {
        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(TelegramClient::TCP_ADDR).unwrap();
        let mut stream = connector.connect("api.telegram.org", stream).unwrap();
        let mut request = String::new();
        request.push_str(&format!("GET /{}{}{} HTTP/1.1\r\n", "bot", &self.api_key, TelegramClient::GET_UPDATE));
        request.push_str(&format!("Host: {}\r\n", TelegramClient::HOST));
        request.push_str("Accept: application/json\r\n");
        request.push_str("Connection: close\r\n\r\n");
        println!("Request {}", request);
        stream.write(request.as_bytes());
        stream.flush();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]);
        func(request.to_string())
    }
}
