use anyhow::{Error, anyhow, bail};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    Add(u8), // +    Use complement to subtract (i.e. -2==+254 (mod 256))
    Ptr(isize), // > and <
    LoopBegin(usize), // [    Index after matching ]
    LoopEnd(usize), // ]    Index after matching [
    Out,
    In,
}

fn parse(source: String) -> Result<Vec<Instr>, Error> {
    let mut program: Vec<_> = source
        .chars()
        .filter_map(|c|
            match c {
                '+' => Some(Instr::Add(1)),
                '-' => Some(Instr::Add(0u8.wrapping_sub(1))),
                '>' => Some(Instr::Ptr(1)),
                '<' => Some(Instr::Ptr(-1)),
                '[' => Some(Instr::LoopBegin(0)),
                ']' => Some(Instr::LoopEnd(0)),
                '.' => Some(Instr::Out),
                ',' => Some(Instr::In),
                _ => None
            }
        )
        .coalesce(|a, b|
            match (a, b) {
                (Instr::Add(c), Instr::Add(d)) => Ok(Instr::Add(c.wrapping_add(d))),
                (Instr::Ptr(c), Instr::Ptr(d)) => Ok(Instr::Ptr(c+d)),
                _ => Err((a, b))
            }
        ).collect(); // loosely inspired by https://stackoverflow.com/a/32717990
        
        let mut jump_stack: Vec<_> = Vec::new();
        
        for i in 0..program.len() {
            match program[i] {
                Instr::LoopBegin(_) => jump_stack.push(i),
                Instr::LoopEnd(_) => {
                    let other = jump_stack.pop()
                        .ok_or(
                            anyhow!("Unmatched closing bracket (`}}`) at position {}", i)
                        )?;
                    // do not jump to matching bracket, instead, jump to instruction
                    // after that to skip an unnecessary comparison
                    program[i] = Instr::LoopEnd(other+1);
                    program[other] = Instr::LoopBegin(i+1);
                }
                _ => ()
            }
        }
        
        let len = jump_stack.len();
        if len != 0 {
            bail!("{} unmatched opening brackets (`{{`)", len);
        }
        
        Ok(program)
    
}

#[cfg(test)]
mod tests {
    use super::{*, Instr::*};

    #[test]
    fn working_1() {
        assert_eq!(
            parse(String::from("+++")).unwrap(),
            [Add(3)]);
    }
    #[test]
    fn working_2() {
        assert_eq!(
            parse(String::from("---")).unwrap(),
            [Add(0u8.wrapping_sub(3))]
        );
    }
    #[test]
    fn working_3() {
        assert_eq!(
            parse(String::from("++>>[--<<]")).unwrap(),
            [Add(2), Ptr(2), LoopBegin(6), Add(0u8.wrapping_sub(2)), Ptr(-2), LoopEnd(3)]
        );
    }
}
