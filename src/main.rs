use crate::vm::Vm;
use std::env::args;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, stdout, Write};
use std::process::exit;

mod chunk;
mod compile;
mod debug;
mod object;
mod scanner;
mod value;
mod vm;

fn main() {
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
}

fn repl(vm: &mut Vm) {
    let stdin = io::stdin();
    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        vm.interpret_src(&line.unwrap());
        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run_file(vm: &mut Vm, path: &str) {
    let source = std::io::read_to_string(File::open(path).unwrap()).unwrap();
    let result = vm.interpret_src(&source);
}
