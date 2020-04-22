use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    // プログラム名がVecの[0]だから1から指定
    let query = &args[1];
    let file_name = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_name);
}
