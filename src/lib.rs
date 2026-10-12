pub fn search<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    content
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a> (query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    let query: String = query.to_lowercase();

    content
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct"; 
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert!(
            search(query, content)
                .eq(["safe, fast, productive."])
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert!(
            search_case_insensitive(query, content)
                .eq(["Rust:", "Trust me."])
        );
    }
}
