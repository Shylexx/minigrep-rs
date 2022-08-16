use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap();

    println!("Searching for {}", config.query);
    println!("In File Path {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Could Not Read File!");

    println!("With Text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
