use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::Read;

fn get_file_string(filename: &str) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut s = String::new();

    try!(file.read_to_string(&mut s));
    Ok(s)
}

fn main() {
    let args = env::args().skip(1);
    for filename in args {
        match get_file_string(&filename) {
            Ok(s) => println!("{}", s),
            Err(msg) => panic!("Error getting file.")
        }
    }
}
