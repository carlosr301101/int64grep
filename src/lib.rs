pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    for line in contents.lines(){
        
        if line.contains(query){
            results.push(line.trim());
        }

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

    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }
    
    #[test]
    fn case_insensitive(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, proDuctive.
        Pick three.";

        assert_eq!(vec!["safe, fast, proDuctive."], search_case_insensitive(query, contents));

    }
    
}