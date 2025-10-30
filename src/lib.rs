pub fn no_parameter() -> &'static str {
    "El programa espera al menos 2 parametros -> [query] [filepath]\nTry using --help\n"
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines(){
        
        if line.contains(query){
            results.push(line.trim());
        }

    }
    if results.is_empty(){
        results.push("No hay ningun match...");
    }
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];
    let  query_lower = &query.to_lowercase();
    
    for line in contents.lines(){
        
        if line.to_lowercase().contains(query_lower){
            results.push(line.trim());
        }

    }
    if results.is_empty(){
        results.push("No hay ningun match...");
    }

    results
}

pub fn count (results_no_count:Vec<&str>) -> Vec<String> {
    let mut numbered_results:Vec<String> = vec![];

    for (i,line) in results_no_count.iter().enumerate(){
        numbered_results.push(format!("{}: {}",i+1,line).to_string());
    }

    if numbered_results.is_empty(){
        numbered_results.push("No hay ningun match...".to_string());
    }
    numbered_results
}

pub fn line_count(){
    unimplemented!()
}

#[cfg(test)]
mod tests{
    use std::vec;

    use super::*;

    #[test]
    fn test_one_result(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }
    
    #[test]
    fn test_case_insensitive(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, proDuctive.
        Pick three.";

        assert_eq!(vec!["safe, fast, proDuctive."], search_case_insensitive(query, contents));

    }

    #[test]
    fn test_no_entrada(){
        assert_eq!("El programa espera al menos 2 parametros -> [query] [filepath]\nTry using --help\n",no_parameter());

    }

    #[test]
    fn test_no_matches() {
        let query = "www";
        let contents= "\
        Rust:
        safe, fast, proDuctive.
        Pick three.";
        assert_eq!(vec!["No hay ningun match..."],search(query, contents));
        assert_eq!(vec!["No hay ningun match..."],search_case_insensitive(query, contents));

    }

    #[test]
    fn test_seach_count() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let results= search(query, contents);
        assert_eq!(vec!["1: safe, fast, productive."],count(results));

        let query = "st";
        let results= search(query, contents);
        assert_eq!(vec!["1: Rust:", "2: safe, fast, productive."],count(results));
    }

    #[test]
    #[ignore = "Aun no esta implementado."]
    fn test_line_count() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let results= search(query, contents);
        // assert_eq!(vec!["2: safe, fast, productive."],line_count(results));

        let query = "st";
        let results= search(query, contents);
        // assert_eq!(vec!["1: Rust:", "2: safe, fast, productive."],line_count(results));
        // unimplemented!();
    }
    
    
}