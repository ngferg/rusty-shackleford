use md_to_html::convert_md_to_html;

#[test]
fn test_file() {
    let expected_html = "<!DOCTYPE html>\n<html>\n<head>\n<title>test-file</title>\n</head>\n<body>\n<h1>This should be an h1</h1>\n<p>This is just plain ol text</p>\n<br/>\n<hr/>\n<br/>\n<h2>This should be an h2</h2>\n<h3>This should be an h3</h3>\n<h4>This should be an h4</h4>\n<h5>This should be an h5</h5>\n<h6>This should be an h6</h6>\n<br/>\n<br/>\n<ul>\n<li>unordered</li>\n<li>things</li>\n<li>in</li>\n<li>a</li>\n<li>list</li>\n</ul>\n<br/>\n<br/>\n</body>\n</html>\n".to_string();

    let html = convert_md_to_html(&"test-file.md".to_string()).unwrap();

    assert_eq!(expected_html, html);
}

#[test]
fn test_nonexisting_file_error() {
    let result = convert_md_to_html(&"some-bad-file.md".to_string());

    assert!(result.is_err());
}

#[test]
fn test_non_md_file_error() {
    let result = convert_md_to_html(&"cargo.toml".to_string());

    assert!(result.is_err());
}
