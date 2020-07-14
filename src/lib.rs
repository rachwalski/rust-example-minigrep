use std::env;
use std::fs;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    // Tests case sensitive behavior of the program using the 'search' method.
    //
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // Tesets case insensitive behavior of the program implemented in the
    // 'search_case_insensitive' method.
    //
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], 
                   search_case_insensitive(query, contents));
    }

}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
} // no semicolon after struct definition

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // no 'return' keyword needed
        Ok(Config { 
            query, 
            filename, 
            case_sensitive 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) // Note no semicolon!
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // idiomatic way of saying we're calling run for it's side effects only
    Ok(())
}

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    //let query = query.to_lowercase();
    //let mut results = Vec::new();
    //
    //for line in contents.lines() {
    //    if line.to_lowercase().contains(&query) {
    //        results.push(line);
    //    }
    //}
    //
    //results

    // The iterator-based piece of code below evaluates to the same thing as the
    // loop-based approach above. This is the preferred way to implement such
    // behavior by Rust programmers. It turns out that they're nearly the same
    // performance-wise, with the iterator-based solution being slightly faster.
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
