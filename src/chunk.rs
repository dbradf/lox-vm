pub enum OpCode {
    OpConstant {index: usize},
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
    pub constants: Vec<f64>,
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

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.push(value);
        self.constants.len() - 1

    }
}