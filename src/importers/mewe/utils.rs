use std::collections::HashMap;
use regex::Regex;

///
/// ```
/// use std::collections::HashMap;
/// use rust_any2feed::importers::mewe::render_content::format_url;
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
    return url.to_string();
}
