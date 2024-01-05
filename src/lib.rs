use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Self, &str> { // Note: if there is an error you might need to make this static. we just want to know why it would fail
      if args.len() < 3 {
          return Err("not enough arguments!");
      }

      let query = args[1].clone();
      let file_path = args[2].clone();

      Ok(Self { query, file_path })
  }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.file_path)?;

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }
}
