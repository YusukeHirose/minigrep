use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let mut f = File::open(config.file_name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_name: String,
}

fn parse_config(args: &[String]) -> Config {
    // プログラム名がVecの[0]だから1から指定
    let query = &args[1].clone();
    let file_name = &args[2].clone();
    Config { query, file_name }
}
