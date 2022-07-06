use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn get_headers(header_args: Vec<String>) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();
    for header_string in header_args.iter() {
        let mut header_iter = header_string.splitn(2, "=");
        let header_key = match header_iter.next() {
            None => {
                return Err("Invalid header format (Key=Value).".parse().unwrap());
            },
            Some(s) => s
        };
        let header_value = match header_iter.next() {
            None => {
                return Err("Invalid header format (Key=Value).".parse().unwrap());
            },
            Some(s) => s
        };
        headers.insert(
            HeaderName::from_str(header_key).unwrap(),
            HeaderValue::from_str(header_value).unwrap()
        );
    }
    return Ok(headers);
}
