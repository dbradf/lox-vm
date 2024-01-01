use crate::chunk::OpCode::{OpAdd, OpConstant, OpDivide, OpNegate, OpReturn};
use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::vm::Vm;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process::exit;

mod chunk;
mod compile;
mod debug;
mod scanner;
mod value;
mod vm;

fn main() {
    // let mut chunk = Chunk::new();
    //
    // let constant = chunk.add_constant(1.2);
    // chunk.write_chunk(OpConstant { index: constant }, 123);
    //
    // let constant = chunk.add_constant(3.4);
    // chunk.write_chunk(OpCode::OpConstant { index: constant}, 123);
    // chunk.write_chunk(OpAdd, 123);
    //
    // let constant = chunk.add_constant(5.6);
    // chunk.write_chunk(OpCode::OpConstant { index: constant}, 123);
    //
    // chunk.write_chunk(OpDivide, 123);
    // chunk.write_chunk(OpNegate, 123);
    // chunk.write_chunk(OpReturn, 123);

    let mut vm = Vm::new();
    let mut args = args();
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, args.nth(1).unwrap().as_mut_str()),
        _ => {
            println!("Usage: clox [path]");
            exit(64);
        }
    }

    vm.run();

    // disassemble_chunk(&chunk, "test chunk");
}

fn repl(vm: &mut Vm) {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        vm.interpret_src(&line.unwrap());
        print!("> ");
    }
}

fn run_file(vm: &mut Vm, path: &str) {
    let source = std::io::read_to_string(File::open(path).unwrap()).unwrap();
    let result = vm.interpret_src(&source);
}
