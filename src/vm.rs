use crate::chunk::{Chunk, OpCode};
use crate::compile::Parser;
use crate::debug::print_value;
use crate::value::Value;

pub struct Vm {
    chunk: Chunk,
    stack: Vec<Value>,
    ip: usize,
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
    Greater,
    Less,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            // chunk,
            stack: vec![],
            ip: 0,
        }
    }

    pub fn interpret_src(&mut self, source: &str) -> InterpretResult {
        let mut parser = Parser::new(source);
        let chunk = parser.compile();
        self.chunk = chunk;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        for c in &self.chunk.code.clone() {
            self.ip += 1;
            let result = match c {
                OpCode::OpConstant { index } => {
                    let constant = self.read_constant(*index);
                    self.stack.push(constant);
                    InterpretResult::Ok
                }
                OpCode::OpReturn => {
                    print_value(&self.stack.pop().unwrap_or_default());
                    println!();
                    InterpretResult::Ok
                }
                OpCode::OpNegate => match self.peek(0) {
                    Value::Number(n) => {
                        self.stack.pop();
                        self.stack.push(Value::Number(-n));
                        InterpretResult::Ok
                    }
                    _ => {
                        self.runtime_error("Operand must be a number.");
                        InterpretResult::RuntimeError
                    }
                },
                OpCode::OpAdd => self.binary_op(BinaryOp::Add),
                OpCode::OpSubtract => self.binary_op(BinaryOp::Subtract),
                OpCode::OpMultiply => self.binary_op(BinaryOp::Multiply),
                OpCode::OpDivide => self.binary_op(BinaryOp::Divide),
                OpCode::OpNil => {
                    self.stack.push(Value::Nil);
                    InterpretResult::Ok
                }
                OpCode::OpTrue => {
                    self.stack.push(Value::Bool(true));
                    InterpretResult::Ok
                }
                OpCode::OpFalse => {
                    self.stack.push(Value::Bool(false));
                    InterpretResult::Ok
                }
                OpCode::OpNot => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(value.is_falsy()));
                    InterpretResult::Ok
                }
                OpCode::OpEqual => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    self.stack.push(Value::Bool(a.equals(&b)));
                    InterpretResult::Ok
                }
                OpCode::OpGreater => self.binary_op(BinaryOp::Greater),
                OpCode::OpLess => self.binary_op(BinaryOp::Less),
            };

            match result {
                InterpretResult::Ok => {}
                _ => {
                    return result;
                }
            }
        }

        return InterpretResult::CompileError;
    }

    fn binary_op(&mut self, op: BinaryOp) -> InterpretResult {
        if !self.peek(0).is_number() || !self.peek(1).is_number() {
            self.runtime_error("Operands must be numbers.");
            return InterpretResult::RuntimeError;
        }

        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        match a {
            Value::Number(a) => match b {
                Value::Number(b) => {
                    self.stack.push(match op {
                        BinaryOp::Add => Value::Number(a + b),
                        BinaryOp::Subtract => Value::Number(a - b),
                        BinaryOp::Multiply => Value::Number(a * b),
                        BinaryOp::Divide => Value::Number(a / b),
                        BinaryOp::Greater => Value::Bool(a < b),
                        BinaryOp::Less => Value::Bool(a > b),
                    });
                    return InterpretResult::Ok;
                }
                _ => {}
            },
            _ => {}
        }
        InterpretResult::RuntimeError
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - distance - 1].clone()
    }

    fn read_constant(&self, index: usize) -> Value {
        self.chunk.constants[index].clone()
    }

    fn runtime_error(&self, message: &str) {
        eprintln!("{}", message);
        let instruction = self.chunk.lines[self.ip - 1].clone();
        eprintln!("[line {}] in script.", instruction);
    }
}
