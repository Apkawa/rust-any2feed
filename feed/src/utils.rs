
///
/// ```
/// use feed::utils::escape;
/// let s = escape("foo&bar=1");
/// assert_eq!(s.as_str(), "foo&amp;bar=1");
/// ```
pub fn escape(s: &str) -> String {
    // TODO check escapes
    s.replace('&', "&amp;")
}