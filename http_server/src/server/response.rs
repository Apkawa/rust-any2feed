use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use bytes::Bytes;


#[derive(Debug, Default)]
pub struct HTTPResponse {
    pub status: u16,
    pub content: Option<Bytes>,
    pub content_type: Option<String>,
    pub headers: HashMap<String, String>,
}

impl HTTPResponse {
    pub fn new(status: u16) -> Self {
        HTTPResponse{
            status,
            ..HTTPResponse::default()
        }
    }
    pub fn with_content(content: String) -> Self {
        HTTPResponse{
            status: 200,
            content: Some(Bytes::from(content)),
            ..HTTPResponse::default()
        }
    }
}

impl Display for HTTPResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let content = &Bytes::new();
        let content = self.content.as_ref().unwrap_or(content);

        let mut extra_headers: Vec<String> = Vec::with_capacity(2);
        if !content.is_empty() {
            let content_type = &"text/plain".to_string();
            let content_type = self.content_type.as_ref()
                    .unwrap_or(content_type);
            extra_headers.push(format!("Content-Type: {}", content_type));
            extra_headers.push(format!("Content-Length: {}", content.len()));
        }

        // TODO корректная обработка HEAD


        let headers = self.headers.iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .chain(extra_headers)
            .collect::<Vec<String>>()
            .join("\r\n");

        write!(f, "HTTP/1.1 {status}\r\n{headers}\r\n\r\n",
                               status = self.status,
                               headers = headers)
    }
}