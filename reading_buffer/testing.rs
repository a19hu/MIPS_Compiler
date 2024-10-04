use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("test.asm")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?; // Unwrap the Result
        println!("{}", line);
        let binary_instru = _intru_to_bin(&line); // Pass a string slice
        println!("{:?}", binary_instru); // Print the binary instruction
    }

    Ok(())
}

fn _intru_to_bin(instru: &str) -> String {
    // This is where you convert the instruction to binary.
    // For now, let's just return a placeholder string.
    format!("Binary representation of: {}", instru)
}

