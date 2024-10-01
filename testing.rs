use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.asm"; // Input file containing MIPS assembly code

    // Open the input file
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    // Define the opcode and function codes for the MIPS instructions
    let mut opcode_map = HashMap::new();
    opcode_map.insert("add", ("000000", "100000"));
    opcode_map.insert("sub", ("000000", "100010"));
    opcode_map.insert("and", ("000000", "100100"));
    opcode_map.insert("or", ("000000", "100101"));
    opcode_map.insert("slt", ("000000", "101010"));
    opcode_map.insert("addi", ("001000", ""));
    opcode_map.insert("lw", ("100011", ""));
    opcode_map.insert("sw", ("101011", ""));
    opcode_map.insert("beq", ("000100", ""));
    opcode_map.insert("j", ("000010", ""));

    // Register mapping for MIPS registers
    let mut register_map = HashMap::new();
    register_map.insert("$zero", "00000");
    register_map.insert("$t0", "01000");
    register_map.insert("$t1", "01001");
    register_map.insert("$t2", "01010");
    register_map.insert("$t3", "01011");
    register_map.insert("$t4", "01100");
    register_map.insert("$t5", "01101");
    register_map.insert("$t6", "01110");
    register_map.insert("$t7", "01111");
    register_map.insert("$s0", "10000");
    register_map.insert("$s1", "10001");
    register_map.insert("$s2", "10010");
    register_map.insert("$s3", "10011");

    // Read the file line by line and process each MIPS instruction
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let instruction = tokens[0]; // First token is the instruction (e.g., add, lw)

        match instruction {
            // Handle R-type instructions
            "add" | "sub" | "and" | "or" | "slt" => {
                let (opcode, funct) = opcode_map[instruction];
                let rs = register_map[tokens[2]]; // second operand
                let rt = register_map[tokens[3]]; // third operand
                let rd = register_map[tokens[1]]; // destination register
                let shamt = "00000"; // shift amount (unused for these operations)
                println!(
                    "{} {} {} {} {} {}",
                    opcode, rs, rt, rd, shamt, funct
                );
            }

            // Handle I-type instructions
            "addi" | "lw" | "sw" | "beq" => {
                let (opcode, _) = opcode_map[instruction];
                let rt = register_map[tokens[1]]; // target register
                let rs = register_map[tokens[2]]; // base register or first operand
                let immediate = format!("{:016b}", tokens[3].parse::<i16>().unwrap());
                println!("{} {} {} {}", opcode, rs, rt, immediate);
            }

            // Handle J-type instruction
            "j" => {
                let (opcode, _) = opcode_map[instruction];
                let address = format!("{:026b}", tokens[1].parse::<u32>().unwrap());
                println!("{} {}", opcode, address);
            }

            _ => println!("Unsupported instruction: {}", instruction),
        }
    }
}
