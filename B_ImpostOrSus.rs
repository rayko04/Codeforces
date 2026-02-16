use std::io::{self, Read};

fn sol(res: &mut Vec<i32>, s: &str) {
    
    let mut count = 0;
    if s.ends_with('u') {
        count += 1;
    }
    if s.starts_with('u') {
        count += 1;
    }

    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut i = 2;

    while i < n-1 {
        if bytes[i-1] == b'u' && bytes[i] == b'u' {
            count += 1;
            i += 1;
        }
        i += 1;
    }

    res.push(count);
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut it = s.split_whitespace();
    let t: i32 = it.next().unwrap().parse().unwrap();

    let mut res : Vec<i32> = Vec::new();

    for _ in 0..t {
        let st = it.next().unwrap();
        sol(&mut res, st);
    }

    for r in res {
        println!("{}", r);
    }
}
