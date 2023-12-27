use crate::chunk::{Chunk, OpCode};
use crate::chunk::OpCode::{OpAdd, OpConstant, OpDivide, OpNegate, OpReturn};
use crate::debug::disassemble_chunk;
use crate::vm::Vm;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpConstant { index: constant }, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OpConstant { index: constant}, 123);
    chunk.write_chunk(OpAdd, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OpConstant { index: constant}, 123);

    chunk.write_chunk(OpDivide, 123);
    chunk.write_chunk(OpNegate, 123);
    chunk.write_chunk(OpReturn, 123);

    let mut vm = Vm::new(chunk);
    vm.interpret();

    // disassemble_chunk(&chunk, "test chunk");
}
