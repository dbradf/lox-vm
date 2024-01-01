use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    let mut prev_line = None;
    chunk
        .code
        .iter()
        .zip(chunk.lines.iter())
        .for_each(|(c, l)| {
            offset = disassemble_instruction(chunk, c, offset, l, &prev_line);
            prev_line = Some(l);
        });
}

fn disassemble_instruction(
    chunk: &Chunk,
    c: &OpCode,
    offset: usize,
    line: &usize,
    prev_line: &Option<&usize>,
) -> usize {
    print!("{:0>4} ", offset);
    if prev_line == &Some(line) {
        print!("   | ");
    } else {
        print!("{:>4} ", line);
    }

    match c {
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        OpCode::OpConstant { index } => constant_instruction("OpConstant", chunk, index, offset),
        OpCode::OpNegate => simple_instruction("OpNegate", offset),
        OpCode::OpAdd => simple_instruction("OpAdd", offset),
        OpCode::OpSubtract => simple_instruction("OpSubtract", offset),
        OpCode::OpMultiply => simple_instruction("OpMultiply", offset),
        OpCode::OpDivide => simple_instruction("OpDivide", offset),
        OpCode::OpNil => simple_instruction("OpNil", offset),
        OpCode::OpTrue => simple_instruction("OpTrue", offset),
        OpCode::OpFalse => simple_instruction("OpFalse", offset),
        OpCode::OpNot => simple_instruction("OpNot", offset),
        OpCode::OpEqual => simple_instruction("OpEqual", offset),
        OpCode::OpGreater => simple_instruction("OpGreater", offset),
        OpCode::OpLess => simple_instruction("OpLess", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, index: &usize, offset: usize) -> usize {
    print!("{} {} '", name, index);
    print_value(&chunk.constants[*index]);
    println!("'");
    offset + 2
}

pub fn print_value(value: &Value) {
    match value {
        Value::Bool(b) => {
            print!("{}", b);
        }
        Value::Nil => {
            print!("<nil>");
        }
        Value::Number(n) => {
            print!("{}", n);
        }
    }
}
