
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


// trim()  The trim function in Rust is a method of the String type
//  that removes leading and trailing whitespace characters from a string
fn main() -> std::io::Result<()> {
    let file = File::open("test1.asm")?; 
    // let file = File::open("test2.asm")?; 

    let reader = io::BufReader::new(file);

    let mut data_section = HashMap::new();
    let mut text_section = vec![];
    let mut labels = HashMap::new();
    let mut in_data_section = false;
    let mut in_text_section = false;
    let mut line_num = 0;
    

    for line in reader.lines() {
        let line = line?.trim().to_string();


        if line.starts_with(".data") {
            in_data_section = true;
            in_text_section = false;
            continue;
        }

        if line.starts_with(".text") {
            in_data_section = false;
            in_text_section = true;
            continue;
        }
        if in_data_section {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == ".word" {
                let label = parts[0].replace(":", "");
                let value: i32 = parts[2].parse().unwrap_or(0);
                data_section.insert(label, value);
            }
        }

        if in_text_section {
            if line.ends_with(":") {
                let label = line.replace(":", "");
                labels.insert(label, line_num);
            } else {
                text_section.push(line.clone());
                line_num += 1;
            }
        }

    }
    for (pc, line) in text_section.iter().enumerate() {
        let binary_instr = instruction_to_binary(&line, &data_section, &labels);
        println!("PC: {}, Instruction: {}\nBinary: {}", pc, line,binary_instr);
    }

    Ok(())
}

fn instruction_to_binary(instru: &str,
    data_section: &HashMap<String, i32>,
    labels: &HashMap<String, usize>) -> String {
    let binding = instru.replace(",", " ");
    let parts: Vec<&str> = binding.split_whitespace().collect();
    let op = parts[0];

    match op {
        "add" | "sub" | "and" | "or" | "slt" => {
            let rs = register_to_binary(parts[2]);
            let rt = register_to_binary(parts[3]);
            let rd = register_to_binary(parts[1]);
            let shamt = "00000";
            let opcode = "000000";

            let funct = match op {
                "add" => "100000",
                "sub" => "100010",
                "and" => "100100",
                "or" => "100101",
                "slt" => "101010",
                _ => "000000",
            };
            format!("{opcode} {rs} {rt} {rd} {shamt} {funct}")
        }
        "addi" => {
            let rt = register_to_binary(parts[1]);
            let rs = register_to_binary(parts[2]);
            let imm = format!("{:016b}", parts[3].parse::<i16>().unwrap());
            let opcode = "001000";
            format!("{opcode} {rs} {rt} {imm}")
        }
        "lw" | "sw" => {
            let rt = register_to_binary(parts[1]);
            let label = parts[2];
            let base_addr = data_section.get(label).unwrap_or(&0);
            let base = "00000"; // Assume base is $zero for now
            let opcode = match op {
                "lw" => "100011",
                "sw" => "101011",
                _ => "000000",
            };
            format!("{opcode} {base} {rt} {base_addr}")
        }
        "beq" => {
            let rs = register_to_binary(parts[1]);
            let rt = register_to_binary(parts[2]);
            let label = parts[3];
            let branch_addr = labels.get(label).unwrap_or(&0);
            let opcode = "000100";
            format!("{opcode} {rs} {rt} {branch_addr}")
        }
        "j" => {
            let label = parts[1];
            let address = labels.get(label).unwrap_or(&0);
            let opcode = "000010";
            format!("{opcode} {:026b}", address)
        }
        _ => format!("Error: Invalid instruction"),
    }
}

fn register_to_binary(reg: &str) -> String {
    let reg_bin = match reg {
        "$Zero" => "00000",
        "$t0" => "01000",
        "$t1" => "01001",
        "$t2" => "01010",
        "$t3" => "01011",
        "$t4" => "01100",
        "$t5" => "01101",
        "$t6" => "01110",
        "$t7" => "01111",
        _ => "00000",
    };

    format!("{}", reg_bin)
}
