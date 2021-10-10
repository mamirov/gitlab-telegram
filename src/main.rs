use std::borrow::{Borrow, Cow};
use std::{env, thread};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::Thread;
use std::time::Duration;
use gitlab_telegram::telegram_client::TelegramClient;
use gitlab_telegram::utils::parse_json;
use gitlab_telegram::webhook::WebHook;

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", "127.0.0.1", 8000)).expect("Couldn't start server");
    let mut incoming = listener.incoming();
    let telegram_client = TelegramClient {
        api_key: env::var("BOT_API_KEY").unwrap()
    };
    thread::spawn(move || {
        loop {
            telegram_client.add_update_listener(|body: String| {
                println!("{}", body);
            })
        }
    });
    loop {
        let stream = incoming.next().unwrap();
        let mut stream = stream.expect("Stream error");
        stream
            .set_read_timeout(Some(Duration::from_millis(10000)))
            .expect("FATAL: Couldn't set read timeout on socket");

        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]);
        let json = parse_json(request);
        let webhook:WebHook = serde_json::from_str(json.as_str()).expect("Cannot parse");
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
