use crate::vm::Value;

use super::Chunk;

const INSTRUCTION_COLOR: &str = "\x1b[32m";
const BOOL: &str = "\x1b[33m";
const INT: &str = "\x1b[34m";
const STR: &str = "\x1b[35m";
const FLOAT: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

macro_rules! simple_instruction {
    ($instruction_name:expr) => {
        eprintln!("{}{}{}", INSTRUCTION_COLOR, $instruction_name, RESET)
    };
}

pub fn print_instructions(instructions: &[Chunk]) {
    for instruction in instructions.iter() {
        match instruction {
            Chunk::CLOCK => simple_instruction!("clock"),
            Chunk::CONST(value) => {
                eprintln!(
                    "{}push{} {}{}",
                    INSTRUCTION_COLOR,
                    RESET,
                    &match value.value {
                        Value::BOOL(v) => format!("{}{}", BOOL, v),
                        Value::INT(v) => format!("{}{}", INT, v),
                        Value::STRING(ref v) => format!("{}\"{}\"", STR, v),
                        Value::FLOAT(v) => format!("{}{}", FLOAT, v),
                    },
                    RESET
                )
            }
            Chunk::ADD => simple_instruction!("add"),
            Chunk::SUB => simple_instruction!("sub"),
            Chunk::DIV => simple_instruction!("div"),
            Chunk::MOD => simple_instruction!("mod"),
            Chunk::MUL => simple_instruction!("mul"),
            Chunk::SMALLER => simple_instruction!("<"),
            Chunk::GREATER => simple_instruction!(">"),
            Chunk::EQUAL => simple_instruction!("="),
            Chunk::EOF => simple_instruction!("eof"),
            Chunk::PRINT => simple_instruction!("print"),
            Chunk::VARIABLEREF(ref s) => eprintln!("{}read{} {}", INSTRUCTION_COLOR, RESET, s),
            Chunk::VARIABLEASS(ref s) => eprintln!("{}push{} {}", INSTRUCTION_COLOR, RESET, s),
            Chunk::ASSIGN => simple_instruction!("assign"),
            Chunk::JMPIFFALSE(index) => {
                eprintln!("{}jmp_if_false{} {}", INSTRUCTION_COLOR, RESET, index)
            }
            Chunk::JMPEND(index) => eprintln!("{}jmp_end{} {}", INSTRUCTION_COLOR, RESET, index),
            Chunk::JMP(index) => eprintln!("{}jmp{} {}", INSTRUCTION_COLOR, RESET, index),
            Chunk::END => simple_instruction!("end"),
            Chunk::IGNORE => simple_instruction!(""),
        }
    }
}
