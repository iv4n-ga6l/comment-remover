use regex::Regex;

/// Removes comments from Python code.
/// Supports single-line (#) and multi-line (''' or """) comments.
pub fn remove_comments(code: &str) -> String {
    // Regex pattern for single-line comments
    let single_line_comment_pattern = r"(?m)#.*$";
    let single_line_comment_regex = Regex::new(single_line_comment_pattern).unwrap();

    // Regex pattern for multi-line comments
    let multi_line_comment_pattern = r"(?s)(""".*?"""|'''.*?''')";
    let multi_line_comment_regex = Regex::new(multi_line_comment_pattern).unwrap();

    // Remove single-line comments
    let code_without_single_line_comments = single_line_comment_regex.replace_all(code, "").to_string();

    // Remove multi-line comments
    let code_without_comments = multi_line_comment_regex.replace_all(&code_without_single_line_comments, "").to_string();

    code_without_comments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_single_line_comments() {
        let input = "print('Hello')  # This is a comment\n# Another comment\nprint('World')";
        let expected = "print('Hello')  \n\nprint('World')";
        assert_eq!(remove_comments(input), expected);
    }

    #[test]
    fn test_remove_multi_line_comments() {
        let input = """
        '''This is a multi-line comment'''
        print('Hello')
        """;
        let expected = "\n        \n        print('Hello')\n        ";
        assert_eq!(remove_comments(input), expected);
    }

    #[test]
    fn test_remove_mixed_comments() {
        let input = """
        # Single-line comment
        print('Hello')
        '''Multi-line comment'''
        print('World')
        """;
        let expected = "\n        \n        print('Hello')\n        \n        print('World')\n        ";
        assert_eq!(remove_comments(input), expected);
    }
}
