use std::borrow::{Borrow, Cow};
use std::io::Read;
use std::net::TcpListener;
use std::time::Duration;
use gitlab_telegram::webhook::WebHook;

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", "127.0.0.1", 8000)).expect("Couldn't start server");
    let mut incoming = listener.incoming();
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
    }
}

fn parse_json(request: Cow<str>) -> String {
    let mut json: String = String::new();
    let mut append = false;
    let mut count = 0;
    for c in request.chars().into_iter() {
        if c.to_string().eq("{") {
            json += &c.to_string();
            count += 1;
            append = true;
            continue
        }
        if c.to_string().eq("}") {
            json += &c.to_string();
            if count == 1 {
                append = false;
            }
            count -= 1;
            continue;
        }
        if append {
            json += &c.to_string();
        }
    }
    json
}
