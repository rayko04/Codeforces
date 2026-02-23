use std::io::{self, Read};

fn sol(res: &mut Vec<i32>, s: &str) {
    let mut count = 0;
    let bytes = s.as_bytes();
    let mut i = 0;

    while i+2 < bytes.len() {
        if i+4 < bytes.len() && &bytes[i..i+5] == b"mapie" {
            count += 1;
            i += 5;
        }
        else if &bytes[i..i+3] == b"pie" || &bytes[i..i+3] == b"map" {
            count += 1;
            i += 3;
        }
        else {
            i += 1;
        }
    }
    res.push(count);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..t {
        let n: i32 = it.next().unwrap().parse().unwrap();
        let s = it.next().unwrap();
        sol(&mut res, s);
    }

    for r in res {
        println!("{}", r);
    }
}
