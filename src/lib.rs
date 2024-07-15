#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

// Note the third parameter, it's a trait - allows for our function to be generic, i.e.
// testable
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            // Not sure about the expect usage here
            writeln!(writer, "{}", line).expect("Writing has failed");
        }
    }
}
