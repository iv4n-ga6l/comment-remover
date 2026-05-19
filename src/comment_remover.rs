use regex::Regex;
use std::collections::HashMap;

/// Global configuration for comment patterns per language.
/// Each language has a tuple of regex patterns for single-line and multi-line comments.
static COMMENT_PATTERNS: &[(&str, &str, &str)] = &[
    ("python", r"(?m)#.*$", r"(?s)(""".*?"""|'''.*?''')"),
    ("go", r"(?m)//.*$", r"(?s)/\*.*?\*/"),
    ("c", r"(?m)//.*$", r"(?s)/\*.*?\*/"),
    ("typescript", r"(?m)//.*$", r"(?s)/\*.*?\*/"),
];

/// Removes comments from code based on the specified language.
///
/// # Arguments
/// * `code` - The source code as a string.
/// * `language` - The programming language of the source code.
///
/// # Returns
/// A string with comments removed.
pub fn remove_comments(code: &str, language: &str) -> String {
    // Find the comment patterns for the specified language.
    let patterns = COMMENT_PATTERNS.iter().find(|&&(lang, _, _)| lang == language);

    if let Some((_, single_line_pattern, multi_line_pattern)) = patterns {
        let single_line_comment_regex = Regex::new(single_line_pattern).unwrap();
        let multi_line_comment_regex = Regex::new(multi_line_pattern).unwrap();

        // Remove single-line comments
        let code_without_single_line_comments = single_line_comment_regex.replace_all(code, "").to_string();

        // Remove multi-line comments
        let code_without_comments = multi_line_comment_regex.replace_all(&code_without_single_line_comments, "").to_string();

        code_without_comments
    } else {
        // If the language is not supported, return the original code unchanged.
        code.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_python_comments() {
        let input = "print('Hello')  # This is a comment\n# Another comment\nprint('World')";
        let expected = "print('Hello')  \n\nprint('World')";
        assert_eq!(remove_comments(input, "python"), expected);
    }

    #[test]
    fn test_remove_go_comments() {
        let input = "package main\n// This is a comment\nfunc main() {\n    /* Multi-line\n       comment */\n    println(\"Hello, World!\")\n}";
        let expected = "package main\n\nfunc main() {\n    \n    println(\"Hello, World!\")\n}";
        assert_eq!(remove_comments(input, "go"), expected);
    }

    #[test]
    fn test_remove_c_comments() {
        let input = "#include <stdio.h>\n// Single-line comment\nint main() {\n    /* Multi-line\n       comment */\n    printf(\"Hello, World!\");\n    return 0;\n}";
        let expected = "#include <stdio.h>\n\nint main() {\n    \n    printf(\"Hello, World!\");\n    return 0;\n}";
        assert_eq!(remove_comments(input, "c"), expected);
    }

    #[test]
    fn test_remove_typescript_comments() {
        let input = "// Single-line comment\nconst greet = () => {\n    /* Multi-line\n       comment */\n    console.log(\"Hello, World!\");\n};";
        let expected = "\nconst greet = () => {\n    \n    console.log(\"Hello, World!\");\n};";
        assert_eq!(remove_comments(input, "typescript"), expected);
    }

    #[test]
    fn test_unsupported_language() {
        let input = "print('Hello')  # This is a comment";
        assert_eq!(remove_comments(input, "ruby"), input);
    }
}