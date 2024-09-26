use std::fs; // read_to_string function in the fs module, 

fn main() {
    let file_contents = fs::read_to_string("poem.txt").expect("Should have been able to read the file");
    println!("info.txt content =\n{file_contents}");

    
}