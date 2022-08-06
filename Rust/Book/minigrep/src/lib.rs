use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        } 
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query, filename, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in lines {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.contains(&query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let q_lower = query.to_lowercase();
    contents.lines()
        .filter(|l| l.to_lowercase().contains(&q_lower))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_lines<'a>(content: &'a str, lines: &[usize]) -> Vec<&'a str> {
        content.lines()
            .enumerate()
            .filter(|(i, _)| lines.contains(i))
            .map(|(_, l)|l)
            .collect()
    }

    fn search_test(content: &str, query: &str, lines: &[usize], case_insensitive: bool) {
        let expected = get_lines(content, lines);

        let output;
        if case_insensitive {
            output = search_case_insensitive(query, content);
        } else {
            output = search(query, content);
        }

        println!("expected:\n{expected:?}\n");
        println!("output:\n{output:?}\n");

        assert_eq!(
            expected.len(), output.len(),
            "Length not equal"
        );
        expected.iter().zip(output.iter()).for_each(|(a, b)| assert_eq!(a, b, "Lines not equal"));
    }

    #[test]
    fn one_result(){
        let query = "s";
        let contents = "
Rust:
safe, fast, productive.
pick three
        ";
        search_test(contents, query, &[1, 2], false);
    }

    #[test]
    fn case_insensitive_test() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
pick three
        ";
        search_test(contents, query, &[1], true);
    }

}