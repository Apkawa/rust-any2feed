use rust_any2feed::server::response::HTTPResponse;

#[test]
fn test_response() {
    let r = HTTPResponse::with_content("OK".to_string());
    assert_eq!(r.status, 200);
    assert_eq!(r.content, Some("OK".to_string()));
    assert_eq!(r.content_type, None);
    assert_eq!(r.to_string(), "");
}
