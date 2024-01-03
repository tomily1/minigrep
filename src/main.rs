use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // dbg!(&args);

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename);

    match contents {
        Ok(contents) => println!("With text:\n{}", contents),
        Err(e) => println!("Something went wrong: {}", e),
    }
        // .expect("Something went wrong reading the file");

    println!("Searching for {}", query);
    println!("In file {}", filename);
    // println!("With text:\n{}", contents);
}
