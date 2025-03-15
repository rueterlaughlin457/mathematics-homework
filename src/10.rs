use std::fs;

fn main() {
    let filename = "example_file.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", contents);
}
