mod jit;

use crate::jit::*;
use std::mem;

const LOAD_CONST: u8 = 0x64;
const STORE_FAST: u8 = 0x7d;
const LOAD_FAST: u8 = 0x7c;
const BINARY_ADD: u8 = 0x17;
const RETURN_VALUE: u8 = 0x53;

enum ParseErr {
    Notemplment,
}

fn run_jit(jit: JitMemory) -> (fn() -> i64) {
    unsafe { mem::transmute(jit.contents) }
}

fn parse() -> Result<(), ParseErr> {
    let bytes = std::fs::read("py/out.b").unwrap();
    let mut jit: JitMemory = JitMemory::new(1);

    let asm: Vec<_> = vec![
        0x66, 0xB8, 0x02, 0x00, 0x66, 0x83, 0xC0, 0x05, 0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut count = 0;

    while count < asm.len() {
        jit[count] = asm[count];
        count += 1;
    }

    let fun = run_jit(jit);
    println!(" ==== {}", fun());

    for byte in bytes.iter() {
        println!("{}", byte);
        if byte == &LOAD_CONST {}
        if byte == &STORE_FAST {}
        if byte == &LOAD_FAST {}
        if byte == &BINARY_ADD {}
        if byte == &RETURN_VALUE {
        } else {
            ParseErr::Notemplment;
        }
    }
    Ok(())
}

fn main() {
    parse();
    println!("Hello World 2")
}
