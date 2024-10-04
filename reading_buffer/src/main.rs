use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("test.asm")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let binary_intru = instruction_to_binary(&line);
        println!("Instruction :{} \nBinary :{:?}", line, binary_intru);
    }

    Ok(())
}

fn instruction_to_binary(instru: &str) -> String {
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
        "lw" | "beq" | "addi" => {
            let rs = register_to_binary(parts[1]);
            let rt = register_to_binary(parts[2]);
            // let imm= parts[3];

            let opcode = match op {
                "lw" => "100011",
                "beq" => "001000",
                "addi" => "000100",
                _ => "000000",
            };
            format!("{opcode}{rs}{rt}")
        }
        "j" => {
            let address = parts[1].parse().unwrap_or(0);
            let opcode = "000010";

            format!("{opcode}{address}")
        }
        _ => format!("something error",),
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
