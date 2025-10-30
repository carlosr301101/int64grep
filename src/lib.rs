//! Small search helpers used by the `int64grep` CLI binary.
//!
//! This crate exposes a few small, focused functions for searching text by line
//! and returning matches either as `&str` slices or as numbered `String` results.
//!
//! The binary `int64grep` demonstrates a tiny CLI wrapper around these helpers.

/// Returns the help message used when the program receives too few arguments.
pub fn no_parameter() -> &'static str {
    "El programa espera al menos 2 parametros -> [query] [filepath]\nTry using --help\n"
}

/// Search `contents` for lines containing `query` (case-sensitive).
///
/// Returns a vector of `&str` slices to the matching, trimmed lines.
/// If no match is found, a single sentinel value `"No hay ningun match..."` is returned.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    if results.is_empty() {
        results.push("No hay ningun match...");
    }
    results
}

/// Case-insensitive search: returns matching trimmed lines as `&str` slices.
///
/// This compares the lowercase form of each line against the lowercase query.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];
    let query_lower = &query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(query_lower) {
            results.push(line.trim());
        }
    }
    if results.is_empty() {
        results.push("No hay ningun match...");
    }

    results
}

/// Search and return numbered results as owned `String`s.
///
/// The returned vector contains strings formatted as `"<n>: <line>"` where `n` is a 1-based index
/// of the matching lines (relative to the list of matches).
pub fn count_search(query: &str, contents: &str) -> Vec<String> {
    let results = search(query, contents);
    let mut numbered_results: Vec<String> = vec![];

    for (i, line) in results.iter().enumerate() {
        numbered_results.push(format!("{}: {}", i + 1, line));
    }

    if numbered_results.is_empty() {
        numbered_results.push("No hay ningun match...".to_string());
    }

    numbered_results
}

/// Number an existing set of result lines and return owned `String`s.
///
/// Useful when you already have a `Vec<&str>` of matching lines and want them numbered.
pub fn count(results_no_count: Vec<&str>) -> Vec<String> {
    let mut numbered_results: Vec<String> = vec![];

    for (i, line) in results_no_count.iter().enumerate() {
        numbered_results.push(format!("{}: {}", i + 1, line));
    }

    if numbered_results.is_empty() {
        numbered_results.push("No hay ningun match...".to_string());
    }

    numbered_results
}

/// Search `contents` and return the original file line numbers with results as `String`.
///
/// This function returns strings formatted as `"<line_number>: <line>"`, where `line_number`
/// is the 1-based index of the line in the original `contents` string. The search is case-insensitive.
pub fn search_line_count(query: &str, contents: &str) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    let q_lower = query.to_lowercase();

    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&q_lower) {
            results.push(format!("{}: {}", i + 1, line.trim()));
        }
    }

    if results.is_empty() {
        results.push("No hay ningun match...".to_string());
    }

    results
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, proDuctive.
        Pick three.";

        assert_eq!(
            vec!["safe, fast, proDuctive."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_no_entrada() {
        assert_eq!(
            "El programa espera al menos 2 parametros -> [query] [filepath]\nTry using --help\n",
            no_parameter()
        );
    }

    #[test]
    fn test_no_matches() {
        let query = "www";
        let contents = "\
        Rust:
        safe, fast, proDuctive.
        Pick three.";
        assert_eq!(vec!["No hay ningun match..."], search(query, contents));
        assert_eq!(
            vec!["No hay ningun match..."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_seach_count() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["1: safe, fast, productive.".to_string()],
            count_search(query, contents)
        );

        let query = "st";
        assert_eq!(
            vec![
                "1: Rust:".to_string(),
                "2: safe, fast, productive.".to_string()
            ],
            count_search(query, contents)
        );
    }

    #[test]
    fn test_line_count() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let results: Vec<_> = search_line_count(query, contents);
        assert_eq!(vec!["2: safe, fast, productive.".to_string()], results);

        let query = "pick";
        let results: Vec<_> = search_line_count(query, contents);
        assert_eq!(vec!["3: Pick three.".to_string()], results);
    }
}
