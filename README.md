# MIPS Processor Simulation

## Introduction

This repository contains a MIPS processor simulator written in Rust. The project was developed as part of a Computer Architecture assignment, focusing on the simulation and testing of MIPS instructions. The simulator supports multiple stages of MIPS instruction execution, including instruction fetch, decode, execution, memory access, and write-back.

### Problem Statement

1. **Task 1**: Implement a MIPS compiler to translate assembly instructions into binary machine code, handling R-type, I-type, and J-type instructions.
2. **Task 2**: Simulate the execution of MIPS binary instructions using a simulated MIPS processor, covering the MIPS datapath, ALU operations, memory access, and branching.
3. **Task 3**: Test the simulator with complex MIPS programs and analyze the results.

### Rust's Benefits

Using Rust for this project provided the following advantages:
- **Memory Safety**: Prevents memory leaks and dangling pointers.
- **Error Handling**: Clear error messages for invalid input due to Rust’s type system.
- **Performance**: Near bare-metal performance for handling large instruction sets.
- **Concurrency**: Support for future extensions with multiple MIPS cores.
- **Type Safety**: Compile-time prevention of common bugs.

## Tasks Overview

### Task 1: MIPS Compiler
The first task involves implementing a MIPS compiler that reads MIPS assembly instructions and translates them into binary machine code.

- **Supported Instructions**: R-type, I-type, and J-type.
- **Input**: MIPS assembly files (e.g., `.asm` files with `.data` and `.text` sections).
- **Output**: Binary machine code suitable for execution
