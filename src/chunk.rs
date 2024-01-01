use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant { index: usize },
    OpNil,
    OpTrue,
    OpFalse,
    OpNegate,
    OpReturn,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn write_chunk(&mut self, op_code: OpCode, line: usize) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
