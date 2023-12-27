use crate::chunk::{Chunk, OpCode};
use crate::debug::print_value;

pub struct Vm {
    chunk: Chunk,
    stack: Vec<f64>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        for c in &self.chunk.code.clone() {
            match c {
                OpCode::OpConstant { index } => {
                    let constant = self.read_constant(*index);
                    self.stack.push(constant);
                }
                OpCode::OpReturn => {
                    print_value(self.stack.pop().unwrap_or_default());
                    println!();
                    return InterpretResult::Ok;
                }
                OpCode::OpNegate => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OpCode::OpAdd => self.binary_op(BinaryOp::Add),
                OpCode::OpSubtract => self.binary_op(BinaryOp::Subtract),
                OpCode::OpMultiply => self.binary_op(BinaryOp::Multiply),
                OpCode::OpDivide => self.binary_op(BinaryOp::Divide),
            }
        }

        return InterpretResult::CompileError;
    }

    fn binary_op(&mut self, op: BinaryOp) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();

        self.stack.push(match op {
            BinaryOp::Add => a + b,
            BinaryOp::Subtract => a - b,
            BinaryOp::Multiply => a * b,
            BinaryOp::Divide =>  a / b,
        })
    }

    fn read_constant(&self, index: usize) -> f64 {
        self.chunk.constants[index]
    }
}
