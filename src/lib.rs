use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_insensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("Not enough arguments enterd");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_insensitive_arg = args.get(3);

    // If both command line arg and environment variable set, prefer command line arg.
    // TODO(HC): Would be nice to avoid using boolan flag as command line arg.
    match case_insensitive_arg {
      Some(case_insensitive) => {
        return Ok(Config {
          query,
          filename,
          // Alternatively find a nicer way of parsing the string to bool.
          case_insensitive: case_insensitive == "true",
        });
      }
      None => {
        return Ok(Config {
          query,
          filename,
          case_insensitive: !env::var("CASE_INSENSITIVE").is_err(),
        });
      }
    }
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file = fs::read_to_string(config.filename)?;

  let results = if config.case_insensitive {
    search_case_insensitive(&config.query, &file)
  } else {
    search(&config.query, &file)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in text.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();
  for line in text.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
DuctDuctDuct";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";

    let contents = "\
Rust
safe, fast, prodrustive.
Pick three.
DuctDuctDuct";

    assert_eq!(
      vec!["Rust", "safe, fast, prodrustive."],
      search_case_insensitive(query, contents)
    );
  }
}
