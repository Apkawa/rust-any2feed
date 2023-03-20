/// ```
/// use test_utils::asserts::remove_extra_spaces;
/// assert_eq!(remove_extra_spaces(r#"
///
///  fooo      bar
/// dfdf
///
/// 2dfd    df     hh
///
///
/// "#), r#"fooo bar
/// dfdf
///
/// 2dfd df hh"#);
/// ```
pub fn remove_extra_spaces(s: &str) -> String {
    let mut output = String::new();
    for line in s.trim().lines() {
        let mut blank = true;
        output.extend(
            line.split_whitespace()
                .inspect(|_| blank = false)
                .flat_map(|word| [word, " "]),
        );

        if !blank {
            // Remove extra trailing ' '
            output.pop();
        } else if !line.is_empty() {
            // For the "   " => " " case
            output.push(' ');
        }
        output.push('\n');
    }

    // Remove trailing '\n'
    output.pop();

    output
}
