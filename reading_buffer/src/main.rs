use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("poem.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
        println!("this is ashutosh");
    }

    Ok(())
}