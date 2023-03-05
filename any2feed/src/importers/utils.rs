use http_server::response;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn response_from_reqwest_response(
    response: reqwest::blocking::Response,
) -> response::HTTPResponse {
    let media_headers: HashMap<String, String> = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect();
    let content_type = media_headers.get("content-type").cloned();
    let status = response.status().as_u16();
    let content = response.bytes().unwrap();
    response::HTTPResponse {
        status,
        content: Some(content),
        content_type,
        headers: media_headers,
    }
}

pub fn timestamp_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
