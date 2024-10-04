use std::fs::File;
use std::io::{self, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("test.asm")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // println!("{}", line);
        let binary_intru= _intru_to_bin(&line);
        println!("{:?}",binary_intru);
    }

    Ok(())
}

fn _intru_to_bin(instru: &str)->String{
    println!("{}",instru.replace(","," "));
    let binding = instru.replace(","," ");
    let types_: Vec<&str>= binding.split_whitespace().collect();
    let op= types_[0];
    print!("{}",op);

    
    format!("Binary representation of: {}", instru)

}


