use std::str::Chars;

enum BFCommand {
    MovR((usize,)),
    MovL((usize,)),
    Add((usize,)),
    Sub((usize,)),
    Out,
    In,
    Jz((usize,)),
    Jnz((usize,)),
}

fn parse(chars: Chars<'_>) -> Vec<BFCommand> {
    let mut out = vec![];
    let mut jumps = vec![];
    for ch in chars {
        use BFCommand as C;
        match ch {
            '>' => {
                if let Some(C::MovR(v)) = out.last_mut() {
                    v.0 += 1;
                } else {
                    out.push(C::MovR((1,)));
                }
            }
            '<' => {
                if let Some(C::MovL(v)) = out.last_mut() {
                    v.0 += 1;
                } else {
                    out.push(C::MovL((1,)));
                }
            }
            '+' => {
                if let Some(C::Add(v)) = out.last_mut() {
                    v.0 += 1;
                } else {
                    out.push(C::Add((1,)));
                }
            }
            '-' => {
                if let Some(C::Sub(v)) = out.last_mut() {
                    v.0 += 1;
                } else {
                    out.push(C::Sub((1,)));
                }
            }
            '.' => out.push(C::Out),
            ',' => out.push(C::In),
            '[' => {
                jumps.push(out.len());
                out.push(C::Jz((0,)));
            }
            ']' => {
                let jmp = jumps.pop().unwrap();
                let mine = out.len();
                if let Some(C::Jz(v)) = out.get_mut(jmp) {
                    v.0 = mine;
                }
                out.push(C::Jnz((jmp,)))
            }
            _ => {}
        }
    }
    out
}

fn run(cmds: &Vec<BFCommand>) {
    let mut mem = [0_u8; 30_000];
    let mut head = 0;
    let mut i = 0;
    while i < cmds.len() {
        use BFCommand as C;
        match *cmds.get(i).unwrap() {
            C::MovR((cells,)) => head += cells,
            C::MovL((cells,)) => head -= cells,
            C::Add((size,)) => mem[head] += size as u8,
            C::Sub((size,)) => mem[head] -= size as u8,
            C::Out => print!("{}", mem[head] as char),
            C::In => todo!(),
            C::Jz((jmp,)) => {
                if mem[head] == 0 {
                    i = jmp;
                }
            }
            C::Jnz((jmp,)) => {
                if mem[head] != 0 {
                    i = jmp;
                }
            }
        };
        i += 1;
    }
}

fn main() {
    run(&parse((include_str!("hello_world.bf")).chars()));
}
