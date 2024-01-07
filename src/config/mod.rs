use std::env;

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Self, &str> { // Note: if there is an error you might need to make this static. we just want to know why it would fail
      if args.len() < 3 {
          return Err("not enough arguments!");
      }

      let query = args[1].clone();
      let file_path = args[2].clone();
      let ignore_case = if args.len() > 3 { args[3].clone() == "true" } else { env::var("IGNORE_CASE").is_ok() };

      Ok(Self { query, file_path, ignore_case })
  }
}
