use std::env;

fn main() {
    // let mut expr = "- + 4 + 2 3 4".split_whitespace();
    println!("{}", eval_stack(&mut env::args().skip(1)));
}

fn _eval_rec(tokens: &mut impl Iterator<Item = String>) -> i32 {
    match tokens.next().unwrap().as_str() {
        "+" => _eval_rec(tokens) + _eval_rec(tokens),
        "-" => _eval_rec(tokens) - _eval_rec(tokens),
        "*" => _eval_rec(tokens) * _eval_rec(tokens),
        "/" => _eval_rec(tokens) / _eval_rec(tokens),
        x => x.parse::<i32>().unwrap(),
    }
}

fn eval_stack(tokens: &mut impl DoubleEndedIterator<Item = String>) -> i32 {
    let mut nums = vec![];
    for t in tokens.rev() {
        match t.as_str() {
            "+" | "-" | "*" | "/" => {
                let first = nums.pop().unwrap();
                let second = nums.pop().unwrap();
                match t.as_str() {
                    "+" => nums.push(first + second),
                    "-" => nums.push(first - second),
                    "*" => nums.push(first * second),
                    "/" => nums.push(first / second),
                    _ => {}
                }
            }
            x => nums.push(x.parse::<i32>().unwrap()),
        }
    }
    nums.pop().unwrap()
}
