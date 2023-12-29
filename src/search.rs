pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut acc: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            acc.push(line);
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Lorem ipsum dolor sit amet,
qui minim labore adipisicing minim
sint cillum sint consectetur cupidatat.
    ";

    #[test]
    fn finds_one_matching_line() {
        let query = "sint";
        assert_eq!(
            vec!["sint cillum sint consectetur cupidatat."],
            search(query, CONTENTS)
        );
    }

    #[test]
    fn finds_multiple_matching_lines() {
        let query = "or";
        assert_eq!(
            vec![
                "Lorem ipsum dolor sit amet,",
                "qui minim labore adipisicing minim"
            ],
            search(query, CONTENTS)
        )
    }

    #[test]
    fn empty_vec_on_no_matching_lines() {
        let query = "nomatch";
        let empty: Vec<&str> = Vec::new();
        assert_eq!(empty, search(query, CONTENTS));
    }
}
