use http_server::{HTTPMethod, HTTPRequest};

#[test]
fn parse_request() {
    let lines: Vec<String> = r#"HEAD / HTTP/1.1
Host: 127.0.0.1:12345
User-Agent: curl/7.74.0
Accept: */*"#
        .lines()
        .map(|l| l.into())
        .collect();
    let res = HTTPRequest::parse(&lines).unwrap();
    assert_eq!(res.method, HTTPMethod::HEAD);
    assert_eq!(res.path, "/");
}

#[test]
fn parse_path() {}
