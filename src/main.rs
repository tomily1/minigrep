use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config: Config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path);

    match contents {
        Ok(content) => println!("With text:\n{}", content),
        Err(e) => println!("Something went wrong: {}", e),
    }
        // .expect("Something went wrong reading the file");

    println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    // println!("With text:\n{:?}", contents);
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
