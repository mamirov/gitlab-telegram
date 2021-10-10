use std::borrow::Cow;

pub fn get_content(buffer: &String) -> String {
    return buffer.to_string().replace("\x00", "").trim().to_string().split("\r\n\r\n").nth(1).unwrap().to_string();
}
