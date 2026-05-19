use comment_remover::remove_comments;

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