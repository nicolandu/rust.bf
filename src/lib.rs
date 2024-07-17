use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::io::{BufWriter, Read, Write};

const INITIAL_CAPACITY: usize = 30000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instr {
    Add(u8),          // +    Use complement to subtract (i.e. -2==+254 (mod 256))
    Ptr(isize),       // > and <
    LoopBegin(usize), // [    Index after matching ]
    LoopEnd(usize),   // ]    Index after matching [
    Out,
    In,
}

pub struct Program {
    instrs: Vec<Instr>,
}

impl Program {
    pub fn parse(source: &str) -> Result<Self> {
        let mut program: Vec<Instr> = source
            .chars()
            .filter_map(|c| match c {
                '+' => Some(Instr::Add(1)),
                '-' => Some(Instr::Add(0u8.wrapping_sub(1))),
                '>' => Some(Instr::Ptr(1)),
                '<' => Some(Instr::Ptr(-1)),
                '[' => Some(Instr::LoopBegin(0)),
                ']' => Some(Instr::LoopEnd(0)),
                '.' => Some(Instr::Out),
                ',' => Some(Instr::In),
                _ => None,
            })
            .coalesce(|a, b| match (a, b) {
                (Instr::Add(c), Instr::Add(d)) => Ok(Instr::Add(c.wrapping_add(d))),
                (Instr::Ptr(c), Instr::Ptr(d)) => Ok(Instr::Ptr(c + d)),
                _ => Err((a, b)),
            })
            .collect(); // loosely inspired by https://stackoverflow.com/a/32717990

        let mut jump_stack = Vec::new();

        for i in 0..program.len() {
            match program[i] {
                Instr::LoopBegin(_) => jump_stack.push(i),
                Instr::LoopEnd(_) => {
                    let other = jump_stack.pop().ok_or(anyhow!(
                        "Unmatched closing bracket (`}}`) at position {}",
                        i
                    ))?;
                    // DO jump to matching bracket, as post-increment will
                    // jump to instruction after that to skip an unnecessary
                    // comparison
                    program[i] = Instr::LoopEnd(other);
                    program[other] = Instr::LoopBegin(i);
                }
                _ => (),
            }
        }

        let len = jump_stack.len();
        if len != 0 {
            bail!("{} unmatched opening brackets (`{{`)", len);
        }

        Ok(Self { instrs: program })
    }

    pub fn run(self, input: &mut impl Read, output: &mut impl Write) -> Result<()> {
        let mut mem = vec![0u8; INITIAL_CAPACITY];
        let mut ptr: usize = 0;
        let mut pc: usize = 0;
        let mut writer = BufWriter::new(output);
        let mut input = input.bytes();
        while pc < self.instrs.len() {
            match self.instrs[pc] {
                Instr::Add(x) => mem[ptr] = mem[ptr].wrapping_add(x),
                Instr::Ptr(x) => {
                    if x >= 0 {
                        let Some(y) = ptr.checked_add(x as usize) else {
                            bail!("BF pointer overflow");
                        };
                        ptr = y;
                    } else {
                        let Some(y) = ptr.checked_sub((-x) as usize) else {
                            bail!("BF pointer underflow");
                        };
                        ptr = y;
                    }
                    if ptr >= mem.len() {
                        mem.resize(mem.len() + 1, 0);
                    }
                }
                Instr::LoopBegin(x) => {
                    if mem[ptr] == 0 {
                        pc = x;
                    }
                }
                Instr::LoopEnd(x) => {
                    if mem[ptr] != 0 {
                        pc = x;
                    }
                }
                Instr::Out => write!(writer, "{}", mem[ptr] as char).unwrap(),
                Instr::In => match input.next() {
                    Some(Ok(v)) => mem[ptr] = v,
                    Some(Err(_)) => bail!("Read error!"),
                    None => mem[ptr] = 0,
                },
            }
            pc += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Instr::*, *};

    #[test]
    fn parse_1() {
        assert_eq!(Program::parse("+++").unwrap().instrs, vec![Add(3)]);
    }
    #[test]
    fn parse_2() {
        assert_eq!(
            Program::parse("---").unwrap().instrs,
            vec![Add(0u8.wrapping_sub(3))]
        );
    }
    #[test]
    fn parse_3() {
        assert_eq!(
            Program::parse("++>>[--<<]").unwrap().instrs,
            vec![
                Add(2),
                Ptr(2),
                LoopBegin(5),
                Add(0u8.wrapping_sub(2)),
                Ptr(-2),
                LoopEnd(2)
            ]
        );
    }
    #[test]
    fn hello_world() {
        let mut buf = Vec::new();
        Program::parse(
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
        ).unwrap().run(&mut "".as_bytes(), &mut buf).unwrap();
        assert_eq!(buf, "Hello World!\n".as_bytes());
    }
    #[test]
    fn cat() {
        let mut buf = Vec::new();
        Program::parse(",[.,]")
            .unwrap()
            .run(&mut "Hello World!".as_bytes(), &mut buf)
            .unwrap();
        assert_eq!(buf, "Hello World!".as_bytes());
    }
}
