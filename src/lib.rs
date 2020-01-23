use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

// Box<dyn Error> == "a type that implements the Error trait"
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let count = word_count(&contents);

    // Sort results (TODO: move to word_count())
    let mut count_vec: Vec<_> = count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (word, freq) in count_vec {
        println!("{}: {}", word, freq);
    }

    Ok(())
}

pub fn word_count<'a>(contents: &'a str) -> HashMap<String, u64> {
    let mut count = HashMap::new();

    // Get rid of all punctuation
    let re = Regex::new(r"[^\w\s]").unwrap();
    let contents = re.replace_all(contents, "");
    
    for line in contents.lines() {
        for word in line.split_whitespace() {
            let word_entry = count.entry(word.to_lowercase()).or_insert(0);
            *word_entry += 1;
        }
    }

    count
}

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // assert_eq! {
        //     vec!["safe, fast, productive."],
        //     // search(contents)
        // };
    }
}
