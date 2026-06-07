use std::fs::File;
use std::io::Read;

fn main() {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(err) => panic!("Failed to open file: {:?}", err),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    println!("{}", file_contents);
}
