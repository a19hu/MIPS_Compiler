use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct DecodedInstruction {
    opcode: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    funct: u8,
    immediate: u16,
    address: u32,
}

struct MipsSimulator {
    registers: [i32; 32],
    memory: HashMap<u32, i32>,
    labels: HashMap<String, u32>,
    pc: u32,
    program: Vec<String>,
    binary_program: Vec<u32>,
    next_free_address: u32,
    reg_map: HashMap<String, usize>,
    reg_map_rev: HashMap<usize, String>,
}

impl MipsSimulator {
    fn new() -> Self {
        let reg_map = HashMap::from([
            ("$zero".to_string(), 0),
            ("$at".to_string(), 1),
            ("$v0".to_string(), 2),
            ("$v1".to_string(), 3),
            ("$a0".to_string(), 4),
            ("$a1".to_string(), 5),
            ("$a2".to_string(), 6),
            ("$a3".to_string(), 7),
            ("$t0".to_string(), 8),
            ("$t1".to_string(), 9),
            ("$t2".to_string(), 10),
            ("$t3".to_string(), 11),
            ("$t4".to_string(), 12),
            ("$t5".to_string(), 13),
            ("$t6".to_string(), 14),
            ("$t7".to_string(), 15),
            ("$s0".to_string(), 16),
            ("$s1".to_string(), 17),
            ("$s2".to_string(), 18),
            ("$s3".to_string(), 19),
            ("$s4".to_string(), 20),
            ("$s5".to_string(), 21),
            ("$s6".to_string(), 22),
            ("$s7".to_string(), 23),
            ("$t8".to_string(), 24),
            ("$t9".to_string(), 25),
            ("$k0".to_string(), 26),
            ("$k1".to_string(), 27),
            ("$gp".to_string(), 28),
            ("$sp".to_string(), 29),
            ("$fp".to_string(), 30),
            ("$ra".to_string(), 31),
        ]);

        let reg_map_rev = reg_map.iter().map(|(k, &v)| (v, k.clone())).collect();

        MipsSimulator {
            registers: [0; 32],
            memory: HashMap::new(),
            labels: HashMap::new(),
            pc: 0,
            program: vec![],
            binary_program: vec![],
            next_free_address: 0x1000,
            reg_map,
            reg_map_rev,
        }
    }

    fn load_program_from_file(&mut self, filename: &str) {
        let file = File::open(filename).expect("Could not open file");
        let reader = BufReader::new(file);
        let mut current_address = 0;

        for line in reader.lines() {
            if let Ok(instruction) = line {
                let instruction = instruction.trim();
                if instruction.starts_with(".data") || instruction.starts_with(".text") {
                    continue;
                }

                if instruction.contains(":") {
                    let label_and_rest: Vec<&str> = instruction.splitn(2, ':').collect();
                    let label = label_and_rest[0].trim().to_string();
                    let rest = label_and_rest.get(1).unwrap_or(&"").trim();

                    if rest.starts_with(".word") {
                        let parts: Vec<&str> = rest.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let value: i32 = parts[1].parse().unwrap_or(0);
                            self.memory.insert(self.next_free_address, value);
                            self.labels.insert(label, self.next_free_address);
                            self.next_free_address += 4;
                        }
                    } else {
                        self.labels.insert(label, current_address);
                        if !rest.is_empty() {
                            self.program.push(rest.to_string());
                            current_address += 1;
                        }
                    }
                    continue;
                }

                if !instruction.is_empty() {
                    self.program.push(instruction.to_string());
                    current_address += 1;
                }
            }
        }
    }

    fn assemble_program(&mut self) {
        for (i, instruction) in self.program.iter().enumerate() {
            let (opcode, parts) = self.instruction_decode_assembly(instruction.clone());
            let binary_instruction =
                self.assemble_instruction(opcode.clone(), parts.clone(), i as u32);

            self.print_binary_instruction(&parts, binary_instruction);

            self.binary_program.push(binary_instruction);
        }
    }

    fn instruction_decode_assembly(&self, instruction: String) -> (String, Vec<String>) {
        let parts: Vec<String> = instruction
            .split_whitespace()
            .map(|p| p.trim_end_matches(',').to_string())
            .collect();
        let opcode = parts[0].clone();
        (opcode, parts)
    }

    fn assemble_instruction(
        &self,
        opcode: String,
        parts: Vec<String>,
        current_address: u32,
    ) -> u32 {
        match opcode.as_str() {
            "add" | "sub" | "and" | "or" | "slt" => {
                let funct_map = HashMap::from([
                    ("add", 32),
                    ("sub", 34),
                    ("and", 36),
                    ("or", 37),
                    ("slt", 42),
                ]);
                let rs = self.get_register_index(&parts[2]) as u32;
                let rt = self.get_register_index(&parts[3]) as u32;
                let rd = self.get_register_index(&parts[1]) as u32;
                let shamt = 0;
                let funct = *funct_map.get(opcode.as_str()).unwrap();
                let binary = (0 << 26)
                    | (rs << 21)
                    | (rt << 16)
                    | (rd << 11)
                    | (shamt << 6)
                    | funct;
                binary
            }
            "addi" => {
                let opcode_num = 8;
                let rs = self.get_register_index(&parts[2]) as u32;
                let rt = self.get_register_index(&parts[1]) as u32;
                let immediate = parts[3].parse::<i16>().unwrap_or(0) as u32 & 0xFFFF;
                let binary = (opcode_num << 26) | (rs << 21) | (rt << 16) | immediate;
                binary
            }
            "lw" | "sw" => {
                let opcode_num = if opcode == "lw" { 35 } else { 43 };
                let rt = self.get_register_index(&parts[1]) as u32;
                let address_part = parts[2].clone();

                if let Some(&label_address) = self.labels.get(&address_part) {
                    let rs = 0;
                    let immediate = (label_address as i32) as u16 as u32;
                    let binary = (opcode_num << 26) | (rs << 21) | (rt << 16) | immediate;
                    binary
                } else if address_part.contains("(") && address_part.contains(")") {
                    let offset_base: Vec<&str> = address_part.split('(').collect();
                    let offset = offset_base[0].parse::<i16>().unwrap_or(0) as u32 & 0xFFFF;
                    let base_reg = offset_base[1].trim_end_matches(')');
                    let rs = self.get_register_index(base_reg) as u32;
                    let binary = (opcode_num << 26) | (rs << 21) | (rt << 16) | offset;
                    binary
                } else {
                    println!("Error: Label {} not found", address_part);
                    0
                }
            }
            "beq" => {
                let opcode_num = 4;
                let rs = self.get_register_index(&parts[1]) as u32;
                let rt = self.get_register_index(&parts[2]) as u32;
                let label = &parts[3];
                if let Some(&label_address) = self.labels.get(label) {
                    let offset =
                        ((label_address as i32 - (current_address as i32 + 1)) as i16) as u16
                            as u32;
                    let binary = (opcode_num << 26) | (rs << 21) | (rt << 16) | offset;
                    binary
                } else {
                    println!("Error: Label {} not found", label);
                    0
                }
            }
            "j" => {
                let opcode_num = 2;
                let label = &parts[1];
                if let Some(&label_address) = self.labels.get(label) {
                    let address = label_address & 0x3FFFFFF;
                    let binary = (opcode_num << 26) | address;
                    binary
                } else {
                    println!("Error: Label {} not found", label);
                    0
                }
            }
            _ => 0,
        }
    }

    fn print_binary_instruction(&self, parts: &Vec<String>, binary_instruction: u32) {
        let opcode = parts[0].clone();
        let bin_str = format!("{:032b}", binary_instruction);
        let formatted_bin = match opcode.as_str() {
            "add" | "sub" | "and" | "or" | "slt" => {
                format!(
                    "{} {} {} {} {} {}",
                    &bin_str[0..6],
                    &bin_str[6..11],
                    &bin_str[11..16],
                    &bin_str[16..21],
                    &bin_str[21..26],
                    &bin_str[26..32]
                )
            }
            "addi" | "lw" | "sw" | "beq" => {
                format!(
                    "{} {} {} {}",
                    &bin_str[0..6],
                    &bin_str[6..11],
                    &bin_str[11..16],
                    &bin_str[16..32]
                )
            }
            "j" => {
                format!("{} {}", &bin_str[0..6], &bin_str[6..32])
            }
            _ => bin_str,
        };

        println!(
            "Instruction: {}\nBinary: {}\n",
            parts.join(" "),
            formatted_bin
        );
    }

    fn instruction_fetch(&mut self) -> Option<u32> {
        if let Some(&binary_instruction) = self.binary_program.get(self.pc as usize) {
            Some(binary_instruction)
        } else {
            None
        }
    }

    fn instruction_decode(&self, binary_instruction: u32) -> DecodedInstruction {
        let opcode = ((binary_instruction >> 26) & 0x3F) as u8;
        let rs = ((binary_instruction >> 21) & 0x1F) as u8;
        let rt = ((binary_instruction >> 16) & 0x1F) as u8;
        let rd = ((binary_instruction >> 11) & 0x1F) as u8;
        let funct = (binary_instruction & 0x3F) as u8;
        let immediate = (binary_instruction & 0xFFFF) as u16;
        let address = binary_instruction & 0x3FFFFFF;
        DecodedInstruction {
            opcode,
            rs,
            rt,
            rd,
            funct,
            immediate,
            address,
        }
    }

    fn execute_instruction(&mut self, decoded: DecodedInstruction) -> bool {
        let opcode = decoded.opcode;
        match opcode {
            0 => {
                let funct = decoded.funct;
                match funct {
                    32 => self.execute_add(decoded),
                    34 => self.execute_sub(decoded),
                    36 => self.execute_and(decoded),
                    37 => self.execute_or(decoded),
                    42 => self.execute_slt(decoded),
                    _ => println!("Unknown funct code: {}", funct),
                }
            }
            8 => self.execute_addi(decoded),
            35 => self.execute_lw(decoded),
            43 => self.execute_sw(decoded),
            4 => {
                self.execute_beq(decoded);
                return true;
            }
            2 => {
                self.execute_j(decoded);
                return true;
            }
            _ => println!("Unknown opcode: {}", opcode),
        }
        false
    }

    fn execute_add(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let rd = decoded.rd as usize;
        self.registers[rd] = self.registers[rs] + self.registers[rt];
    }

    fn execute_sub(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let rd = decoded.rd as usize;
        self.registers[rd] = self.registers[rs] - self.registers[rt];
    }

    fn execute_and(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let rd = decoded.rd as usize;
        self.registers[rd] = self.registers[rs] & self.registers[rt];
    }

    fn execute_or(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let rd = decoded.rd as usize;
        self.registers[rd] = self.registers[rs] | self.registers[rt];
    }

    fn execute_slt(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let rd = decoded.rd as usize;
        self.registers[rd] = if self.registers[rs] < self.registers[rt] {
            1
        } else {
            0
        };
    }

    fn execute_addi(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let immediate = decoded.immediate as i16 as i32;
        self.registers[rt] = self.registers[rs] + immediate;
    }

    fn execute_lw(&mut self, decoded: DecodedInstruction) {
        let base = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let offset = decoded.immediate as i16 as i32;
        let address = (self.registers[base] + offset) as u32;
        if let Some(&value) = self.memory.get(&address) {
            self.registers[rt] = value;
        } else {
            println!("Memory read error at address: 0x{:08X}", address);
        }
    }

    fn execute_sw(&mut self, decoded: DecodedInstruction) {
        let base = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let offset = decoded.immediate as i16 as i32;
        let address = (self.registers[base] + offset) as u32;
        let value = self.registers[rt];
        self.memory.insert(address, value);
    }

    fn execute_beq(&mut self, decoded: DecodedInstruction) {
        let rs = decoded.rs as usize;
        let rt = decoded.rt as usize;
        let offset = decoded.immediate as i16 as i32;
        if self.registers[rs] == self.registers[rt] {
            self.pc = ((self.pc as i32) + offset + 1) as u32;
        } else {
            self.pc += 1;
        }
    }

    fn execute_j(&mut self, decoded: DecodedInstruction) {
        self.pc = decoded.address;
    }

    fn get_register_index(&self, register: &str) -> usize {
        *self.reg_map.get(register).unwrap_or_else(|| {
            println!("Unknown register: {}", register);
            &0
        })
    }

    fn get_register_name(&self, index: usize) -> String {
        self.reg_map_rev
            .get(&index)
            .cloned()
            .unwrap_or_else(|| format!("${}", index))
    }

    fn run(&mut self) {
        while (self.pc as usize) < self.binary_program.len() {
            if let Some(binary_instruction) = self.instruction_fetch() {
                let decoded = self.instruction_decode(binary_instruction);
                let pc_modified = self.execute_instruction(decoded);
                if !pc_modified {
                    self.pc += 1;
                }
            } else {
                break;
            }
        }
    }

    fn print_registers(&self) {
        println!("Registers:");
        for (i, reg) in self.registers.iter().enumerate() {
            let reg_name = self.get_register_name(i);
            println!("{} ({:>2}): {}", reg_name, i, reg);
        }
    }

    fn print_memory(&self) {
        println!("Memory:");
        for (address, value) in &self.memory {
            let mut label_name = String::new();
            for (label, &addr) in &self.labels {
                if addr == *address {
                    label_name = label.clone();
                    break;
                }
            }
            if label_name.is_empty() {
                println!("Address: 0x{:08X}, Value: {}", address, value);
            } else {
                println!("Address: 0x{:08X} ({}), Value: {}", address, label_name, value);
            }
        }
    }
}

fn main() {
    let mut simulator = MipsSimulator::new();

    simulator.load_program_from_file("{put your filename here}");

    simulator.assemble_program();

    simulator.run();

    simulator.print_registers();
    simulator.print_memory();
}
