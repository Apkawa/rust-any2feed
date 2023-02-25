use pulldown_cmark::{Parser, Options, html};


///
/// ```
/// use any2feed::importers::mewe::markdown::md_to_html;
/// let s = "Hello world, this is a ~~complicated~~ *very simple* example.".to_string();
/// let html = md_to_html(&s);
/// let expect_html = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
/// assert_eq!(html, expect_html)
/// ```
pub fn md_to_html(md_text: &String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md_text, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

