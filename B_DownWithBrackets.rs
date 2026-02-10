use std::io::{self, Read};

fn sol(st: &str, res: &mut Vec<String>) {
    let mut stack = Vec::new();
    let mut count = 0;

    for c in st.chars() {
        if stack.is_empty() {
            count += 1;
        }

        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            stack.pop();
        }
    }
    if count > 1 {
        res.push("YES".to_string());
    } else {
        res.push("NO".to_string());
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let t: i32 = it.next().unwrap().parse().unwrap();

    let mut res: Vec<String> = Vec::new();
    for _ in 0..t {
        let st = it.next().unwrap();
        sol(st, &mut res);
    }

    for r in res {
        println!("{}", r);
    }
}
