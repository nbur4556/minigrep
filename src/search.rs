pub fn case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut acc: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
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
            case_sensitive(query, CONTENTS)
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
            case_sensitive(query, CONTENTS)
        )
    }

    #[test]
    fn empty_vec_on_no_matching_lines() {
        let query = "nomatch";
        let empty: Vec<&str> = Vec::new();
        assert_eq!(empty, case_sensitive(query, CONTENTS));
    }

    #[test]
    fn should_find_case_insensitive_matching_lines() {
        let query = "lorem";
        assert_eq!(
            vec!["Lorem ipsum dolor sit amet,"],
            case_insensitive(query, CONTENTS),
        );
    }
}
