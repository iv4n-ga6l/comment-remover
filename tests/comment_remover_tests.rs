use comment_remover::remove_comments;

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
    let expected = "\n    \n    print('Hello')\n    ";
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
    let expected = "\n    \n    print('Hello')\n    \n    print('World')\n    ";
    assert_eq!(remove_comments(input), expected);
}
