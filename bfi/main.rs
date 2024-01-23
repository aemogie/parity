use core::str::Chars;
use std::io::{stdin, stdout, Read, Write};

use crossterm::{cursor::MoveToColumn, terminal, ExecutableCommand};

enum BFCommand {
    MovR(usize),
    MovL(usize),
    Add(u8),
    Sub(u8),
    Out,
    In,
    Jz(usize),
    Jnz(usize),
}

fn parse(chars: Chars<'_>) -> Vec<BFCommand> {
    let mut out = vec![];
    let mut jumps = vec![];
    for ch in chars {
        use BFCommand as C;
        match ch {
            '>' => {
                if let Some(C::MovR(v)) = out.last_mut() {
                    *v += 1;
                } else {
                    out.push(C::MovR(1));
                }
            }
            '<' => {
                if let Some(C::MovL(v)) = out.last_mut() {
                    *v += 1;
                } else {
                    out.push(C::MovL(1));
                }
            }
            '+' => {
                if let Some(C::Add(v)) = out.last_mut() {
                    *v += 1;
                } else {
                    out.push(C::Add(1));
                }
            }
            '-' => {
                if let Some(C::Sub(v)) = out.last_mut() {
                    *v += 1;
                } else {
                    out.push(C::Sub(1));
                }
            }
            '.' => out.push(C::Out),
            ',' => out.push(C::In),
            '[' => {
                jumps.push(out.len());
                out.push(C::Jz(0));
            }
            ']' => {
                let jmp = jumps.pop().unwrap();
                let mine = out.len();
                if let Some(C::Jz(v)) = out.get_mut(jmp) {
                    *v = mine;
                }
                out.push(C::Jnz(jmp));
            }
            _ => {}
        }
    }
    out
}

fn run(cmds: &Vec<BFCommand>) {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    stdout.execute(MoveToColumn(0)).unwrap();
    stdout.flush().unwrap();
    let mut stdin = stdin();
    let mut mem = [0u8; 30_000];
    let mut head = 0;
    let mut i = 0;
    while i < cmds.len() {
        use BFCommand as C;
        match cmds[i] {
            C::MovR(cells) => head += cells,
            C::MovL(cells) => head -= cells,
            C::Add(size) => mem[head] += size,
            C::Sub(size) => mem[head] -= size,
            C::Out => stdout.write(&mem[head..head + 1]).map_or((), |_| ()),
            C::In => stdin.read(&mut mem[head..head + 1]).map_or((), |_| ()),
            C::Jz(jmp) => {
                if mem[head] == 0 {
                    i = jmp;
                }
            }
            C::Jnz(jmp) => {
                if mem[head] != 0 {
                    i = jmp;
                }
            }
        };
        stdout.flush().unwrap();
        i += 1;
    }
    terminal::disable_raw_mode().unwrap();
}

fn main() {
    run(&parse((include_str!("hello_world.bf")).chars()));
    run(&parse("+++[>,-.+.+.<-]".chars()))
}
