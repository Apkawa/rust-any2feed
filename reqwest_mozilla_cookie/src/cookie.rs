use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::io;
use std::io::{Error, ErrorKind};

use reqwest::cookie::{CookieStore, Jar};

use reqwest::Url;

#[allow(clippy::tabs_in_doc_comments)]
/// В формате  Netscape HTTP Cookie File http://fileformats.archiveteam.org/wiki/Netscape_cookies.txt
/// https://curl.se/rfc/cookie_spec.html
/// ```
/// use reqwest::cookie::CookieStore;
/// use reqwest::Url;
/// use reqwest_mozilla_cookie::cookie::import_cookie_from_string;
/// let cookie_str = r###"# Netscape HTTP Cookie File
///
/// kremlin.ru	FALSE	/	FALSE		sid	foo	27
/// .kremlin.ru	FALSE	/foo	FALSE		foo	test	27
/// "###.to_string();
/// let jar = import_cookie_from_string(&cookie_str).unwrap();
/// let url = Url::parse("https://.kremlin.ru").unwrap();
/// let cookies = jar.cookies(&url).unwrap();
/// assert_eq!(cookies.to_str().unwrap(), "sid=foo");
/// ```
pub fn import_cookie_from_string(cookie_str: &str) -> io::Result<Jar> {
    let jar = Jar::default();

    for line in cookie_str.lines().map(|l| l.trim()) {
        let line = line.trim_start_matches("#HttpOnly_");
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let req_head = line.split('\t').collect::<Vec<&str>>();
        let [host, _subdomains, path, _is_secure, _expire, name, value] = match req_head[..] {
            [host, subdomains, path, is_secure, expire, name, value, ..] => {
                [host, subdomains, path, is_secure, expire, name, value]
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid csv file {line}"),
                ));
            }
        };

        let url = format!("https://{host}").parse::<Url>().unwrap();
        jar.add_cookie_str(
            format!("{name}={value}; Domain={host}; Path={path}").as_str(),
            &url,
        );
    }
    Ok(jar)
}

pub fn import_cookie_from_file(path: &String) -> io::Result<Jar> {
    let cookie_str = read_to_string(path)?;
    import_cookie_from_string(&cookie_str)
}

pub fn update_cookie_from_file(jar: &Jar, url: &Url, path: &String) -> Option<()> {
    let cookie_str = read_to_string(path).ok()?;
    let new_cookie_str = merge_cookie_to_string(jar, url, &cookie_str)?;
    write(path, new_cookie_str).ok()
}

pub fn merge_cookie_to_string(jar: &Jar, url: &Url, cookie_txt: &str) -> Option<String> {
    let domain = url.domain().unwrap().trim_start_matches('.');
    let cookies = jar.cookies(url)?;
    let mut cookies_map = cookies
        .to_str()
        .ok()?
        .split(';')
        .map(|s| s.trim().split_once('=').unwrap())
        .collect::<HashMap<&str, &str>>();

    let lines = cookie_txt.lines().map(|l| l.trim());
    let mut new_lines: Vec<String> = Vec::with_capacity(10);
    for line in lines {
        if line.is_empty() || line.starts_with('#') {
            new_lines.push(line.to_string());
            continue;
        }
        let req_head = line.split('\t').collect::<Vec<&str>>();
        let [host, subdomains, path, is_secure, expire, name, value] = match req_head[..] {
            [host, subdomains, path, is_secure, expire, name, value, ..] => {
                [host, subdomains, path, is_secure, expire, name, value]
            }
            _ => {
                return None;
            }
        };
        if host.trim_start_matches('.') == domain {
            let new_value = cookies_map.remove(name).unwrap_or(value);
            let new_head = [host, subdomains, path, is_secure, expire, name, new_value];
            let line = new_head.join("\t");
            new_lines.push(line.to_string());
        } else {
            new_lines.push(line.to_string());
        }
    }
    for (key, value) in cookies_map {
        let line = [domain, "TRUE", "/", "FALSE", "", key, value].join("\t");
        new_lines.push(line.to_string());
    }

    Some(new_lines.join("\n"))
}

#[cfg(test)]
mod test {
    use crate::cookie::{import_cookie_from_string, merge_cookie_to_string};
    use reqwest::cookie::CookieStore;
    use reqwest::Url;

    /// Reordering cookie str for reproduce test
    fn reorder_cookies(cookies_str: &str) -> String {
        let mut v: Vec<_> = cookies_str.split(';').map(|s| s.trim()).collect();
        v.sort();
        v.join("; ")
    }

    #[test]
    fn test_import_cookie_from_string() {
        let cookie_str = r###"# Netscape HTTP Cookie File
        # http://curl.haxx.se/rfc/cookie_spec.html
        # This is a generated file!  Do not edit.

        kremlin.ru	FALSE	/	FALSE		sid	foo	27
        kremlin.ru	FALSE	/	FALSE		bar	baz	27
        .kremlin.ru	FALSE	/foo	FALSE		foo	test	27
        "###
        .to_string();
        let jar = import_cookie_from_string(&cookie_str).unwrap();
        let url = Url::parse("https://.kremlin.ru").unwrap();
        let cookies = jar.cookies(&url).unwrap();
        // TODO reproduce order
        assert_eq!(
            reorder_cookies(cookies.to_str().unwrap()),
            "bar=baz; sid=foo"
        );
    }

    #[test]
    fn test_merge_cookie_from_string() {
        let cookie_str = r###"# Netscape HTTP Cookie File
        # http://curl.haxx.se/rfc/cookie_spec.html
        # This is a generated file!  Do not edit.

        kremlin.ru	FALSE	/	FALSE		sid	foo	27
        kremlin.ru	FALSE	/	FALSE		bar	baz	27
        .kremlin.ru	FALSE	/foo	FALSE		foo	test	27
        "###
        .to_string();
        let url = Url::parse("https://kremlin.ru").unwrap();
        let jar = import_cookie_from_string(&cookie_str).unwrap();
        jar.add_cookie_str("foo=foobaz", &url);
        jar.add_cookie_str("sid=foo2", &url);
        jar.add_cookie_str("lalala=1", &url);
        let new_str = merge_cookie_to_string(&jar, &url, &cookie_str).unwrap();
        // assert_eq!(new_str, "sid=foo; bar=baz");
        let jar = import_cookie_from_string(&new_str).unwrap();
        let cookies = jar.cookies(&url).unwrap();
        assert_eq!(
            reorder_cookies(cookies.to_str().unwrap()),
            "bar=baz; lalala=1; sid=foo2"
        );
    }
}
