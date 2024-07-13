use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Add(u8), // +    Use complement to subtract (i.e. -2==+254 (mod 256))
    Ptr(isize), // > and <
    Do(usize), // [    Index after matching ]
    While(usize), // ]    Index after matching [
    Out,
    In,
}

fn parse(program: String) -> Vec<Instr> {
    program
        .chars()
        .filter_map(|c|
            match c {
                '+' => Some(Instr::Add(1)),
                '-' => Some(Instr::Add(u8::MAX)),
                '[' => Some(Instr::Do(0)),
                ']' => Some(Instr::While(0)),
                '.' => Some(Instr::Out),
                ',' => Some(Instr::In),
                _ => None
            }
        )
        .coalesce(|a, b|
            match (a, b) {
                (Instr::Add(c), Instr::Add(d)) => Ok(Instr::Add(c+d)),
                _ => Err((a, b))
            }
        ).collect() // https://stackoverflow.com/a/32717990
}
