use bytes::Bytes;
use http_server::HTTPResponse;

#[test]
fn test_response() {
    let r = HTTPResponse::with_content("OK");
    assert_eq!(r.status, 200);
    assert_eq!(r.content, Some(Bytes::from("OK")));
    assert_eq!(r.content_type, None);
    // TODO
    //     assert_eq!(r.to_string(), r#"HTTP/1.1 200
    // Content-Type: text/plain
    // Content-Length: 2
    //
    // "#);
}
