use std::collections::HashMap;
use regex::Regex;
use reqwest::Url;

///
/// ```
/// use std::collections::HashMap;
/// use any2feed::importers::mewe::render_content::format_url;
/// let url = "/api/v2/photo/k...E/{imageSize}/img?static={static}";
/// let args: HashMap<&str, &str> = HashMap::from([("imageSize", "200x300"), ("static", "0")]);
/// let result = format_url(url, &args);
/// assert_eq!(result, "/api/v2/photo/k...E/200x300/img?static=0")
/// ```
pub fn format_url(url: &str, args: &HashMap<&str, &str>) -> String {
    // Быстрофункция на регулярках
    let mut url = url.to_string();
    for (k, v) in args {
        let r = Regex::new(format!("\\{{{k}\\}}").as_str()).unwrap();
        url = r.replace_all(url.as_str(), *v).to_string();
    }
    url
}

///
/// ```
/// use std::collections::HashMap;
/// use reqwest::Url;
/// use any2feed::importers::mewe::utils::update_query;
/// let mut url = Url::parse("https://example.com/?foo=1&bar=2&baz=3").unwrap();
/// let query = HashMap::from([("foo", "2"), ("bar", "3")]);
/// update_query(&mut url, &query);
/// // order not guaranteed, reorder for reproduce
/// let mut pairs: Vec<String> = url.query_pairs().map(|(k, v)| format!("{k}={v}")).collect();
/// pairs.sort();
/// assert_eq!(pairs.join("&"), "bar=3&baz=3&foo=2");
/// ```
pub fn update_query(url: &mut Url, query: &HashMap<&str, &str>) {
    let mut query_params: HashMap<String, String> = url.query_pairs()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    for (k, v) in query {
        query_params.insert(k.to_string(), v.to_string());
    }
    url.query_pairs_mut()
        .clear()
        .extend_pairs(query_params);
}