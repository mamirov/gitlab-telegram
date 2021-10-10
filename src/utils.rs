use std::borrow::Cow;

pub fn parse_json(request: Cow<str>) -> String {
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
