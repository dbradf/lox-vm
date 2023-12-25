use crate::chunk::{Chunk};
use crate::chunk::OpCode::{OpConstant, OpReturn};
use crate::debug::disassemble_chunk;

mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpConstant { index: constant }, 123);
    chunk.write_chunk(OpReturn, 123);

    disassemble_chunk(&chunk, "test chunk");
}
