use super::super::searcher::search;

#[test]
fn test_search_case_sensitive() {
    let query = "safe";
    let content = "
Rust:
safe, fast, productive.
Pick three.
";

    assert_eq!(
        vec!["safe, fast, productive."],
        search::search_case_sensitive(query, content)
    );
}

#[test]
fn test_search_case_insensitive() {
    let query = "ruSt";
    let content = "
Rust:
safe, fast, productive.
Pick three.
";

    assert_eq!(
        vec!["Rust:"],
        search::search_case_insensitive(query, content)
    );
}
