use std::collections::HashMap;
use regex::Regex;
use reqwest::Url;

///
/// ```
/// use std::collections::HashMap;
/// use mewe_api::utils::format_url;
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
/// use mewe_api::utils::update_query;
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

///
/// ```
/// use mewe_api::utils::replace_user_mention_to_name;
///
/// let res = replace_user_mention_to_name("@{{u_5c26fe32dfd8gff8f7657c}Пользователь пользователя}");
/// assert_eq!(res, "@Пользователь пользователя".to_string())
/// ```
pub fn replace_user_mention_to_name(text: &str) -> String {
    let re = Regex::new(r#"@\{\{u_(\w+?)}([\w\s]+?)}"#).unwrap();
    re.replace(text, "@$2").to_string()
}

///
/// ```
/// use mewe_api::utils::replace_user_mention_to_html_url;
///
/// let res = replace_user_mention_to_html_url("@{{u_5c26fe32dfd8gff8f7657c}Пользователь пользователя}");
/// assert_eq!(res, r#"<a href="https://mewe.com/i/id=5c26fe32dfd8gff8f7657c">@Пользователь пользователя</a>"#.to_string())
/// ```
pub fn replace_user_mention_to_html_url(text: &str) -> String {
    let re = Regex::new(r#"@\{\{u_(\w+?)}([\w\s]+?)}"#).unwrap();
    re.replace(text, r#"<a href="https://mewe.com/i/id=$1">@$2</a>"#).to_string()
}
