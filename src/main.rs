mod file_reader;
use file_reader::read_input;


fn main() {
    println!("Hello, world!");
    for line in read_input("input/problem1.txt") {
    	println!("{}", line)
    }
}
