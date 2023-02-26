use std::collections::HashMap;


///
/// ```
/// use std::collections::HashMap;
/// use http_server::utils::parse_match_captures;
/// let re = regex::Regex::new("^/foo/(me|bar)(?:/(123)/|)$").unwrap();
/// assert_eq!(parse_match_captures(&re, "/foo/me"),
///     Some(HashMap::from([
///         ("0".to_string(), Some("/foo/me".to_string())),
///         ("1".to_string(), Some("me".to_string())),
///         ("2".to_string(), None),
/// ]
/// )));
/// let c = parse_match_captures(&re, "/foo/bar/123/");
/// assert_eq!(c,
///     Some()
/// );
/// assert_eq!(c.unwrap().get("1").unwrap(), &Some("bar".to_string()));
/// assert_eq!(parse_match_captures(&re, "/foo/baz/565"), None);
/// ```
/// currently no supported named captures https://github.com/rust-lang/regex/issues/955
pub fn parse_match_captures(re: &regex::Regex, text: &str) -> Option<HashMap<String, Option<String>>> {
    let cap = re.captures(text)?;
    let mut res: HashMap<String, Option<String>> = HashMap::with_capacity(cap.len());
    for (i, c) in cap.iter().enumerate() {
        res.insert(i.to_string(), c.map(|c| c.as_str().to_string()));
    }
    Some(res)
}

///
/// ```
/// use std::collections::HashMap;
/// use http_server::utils::path_params_to_vec;
/// let path_params = HashMap::from([
///         ("2".to_string(), Some("123".to_string())),
///         ("1".to_string(), Some("bar".to_string())),
///         ("0".to_string(), Some("/foo/bar/123/".to_string())),
/// ]);
/// assert_eq!(
///     path_params_to_vec(&path_params),
///     vec![
///         Some("/foo/bar/123/".to_string()),
///         Some("bar".to_string()),
///         Some("123".to_string()),
///     ]);
/// ```
pub fn path_params_to_vec(path_params: &HashMap<String, Option<String>>) -> Vec<Option<String>> {
    let mut pairs: Vec<_> = path_params.iter()
        .map(|(k, v)| (k, v))
        .collect();
    pairs.sort();

    pairs.into_iter().map(|(_k, v)| v.clone()).collect()
}
