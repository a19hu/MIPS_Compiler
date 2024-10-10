use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// The trim function in Rust is a method of the String type
// that removes leading and trailing whitespace characters from a string
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

    let mut data_address = 0x10010000; // Base address for data section

    for line in reader.lines() {
        let line = line?;
        // Remove comments
        let line = line.split('#').next().unwrap().trim().to_string();

        if line.is_empty() {
            continue;
        }

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
                data_section.insert(label, data_address);
                data_address += 4;
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
        let binary_instr = instruction_to_binary(&line, &data_section, &labels, pc);
        println!("PC: {}, Instruction: {}\nBinary: {}", pc, line, binary_instr);
    }

    Ok(())
}

fn instruction_to_binary(
    instru: &str,
    data_section: &HashMap<String, u32>,
    labels: &HashMap<String, usize>,
    current_pc: usize,
) -> String {
    let binding = instru.replace(",", " ");
    let parts: Vec<&str> = binding.split_whitespace().collect();

    if parts.is_empty() {
        return "Error: Empty instruction".to_string();
    }

    let op = parts[0];

    match op {
        "add" | "sub" | "and" | "or" | "slt" => {
            if parts.len() < 4 {
                return "Error: Invalid R-type instruction".to_string();
            }
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
            if parts.len() < 4 {
                return "Error: Invalid addi instruction".to_string();
            }
            let rt = register_to_binary(parts[1]);
            let rs = register_to_binary(parts[2]);
            let imm = format!("{:016b}", parts[3].parse::<i16>().unwrap_or(0));
            let opcode = "001000";
            format!("{opcode} {rs} {rt} {imm}")
        }
        "lw" | "sw" => {
            if parts.len() < 3 {
                return "Error: Invalid lw/sw instruction".to_string();
            }
            let rt = register_to_binary(parts[1]);
            let label = parts[2];
            let immediate = data_section.get(label).unwrap_or(&0);
            let rs = "00000"; // Assume base is $zero for now
            let opcode = match op {
                "lw" => "100011",
                "sw" => "101011",
                _ => "000000",
            };
            let imm = format!("{:016b}", immediate);
            format!("{opcode} {rs} {rt} {imm}")
        }
        "beq" => {
            if parts.len() < 4 {
                return "Error: Invalid beq instruction".to_string();
            }
            let rs = register_to_binary(parts[1]);
            let rt = register_to_binary(parts[2]);
            let label = parts[3];
            if let Some(&label_addr) = labels.get(label) {
                // Branch offset calculation: (label_addr - (current_pc + 1))
                let offset = (label_addr as i32 - (current_pc as i32 + 1)) as i16;
                let imm = format!("{:016b}", offset);
                let opcode = "000100";
                format!("{opcode} {rs} {rt} {imm}")
            } else {
                "Error: Undefined label".to_string()
            }
        }
        "j" => {
            if parts.len() < 2 {
                return "Error: Invalid j instruction".to_string();
            }
            let label = parts[1];
            if let Some(&label_addr) = labels.get(label) {
                let address = label_addr;
                let opcode = "000010";
                format!("{opcode} {:026b}", address)
            } else {
                "Error: Undefined label".to_string()
            }
        }
        _ => "Error: Invalid instruction".to_string(),
    }
}

fn register_to_binary(reg: &str) -> String {
    let reg_bin = match reg {
        "$zero" => "00000",
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
    reg_bin.to_string()
}
