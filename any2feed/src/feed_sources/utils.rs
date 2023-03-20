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

// ///```
// /// use any2feed::feed_sources::utils::take_option;
// /// let o = take_option([None, Some("foo")]);
// /// assert_eq!(o, Some("foo"))
// ///```
// pub fn take_option<T, I>(options: I) -> Option<T>
//     where
//         I: IntoIterator<Item=Option<T>>,
//         Option<T>: FromIterator<Option<T>>,
//         T: Sized + ToOwned
//
// {
//     options.into_iter()
//         .filter(Option::is_some)
//         .take(1)
//         .collect()
// }
