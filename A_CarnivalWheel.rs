use std::io::{self, Read};

fn main() {
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut result: Vec<i32> = Vec::with_capacity(t);

    for _ in 0..t {
        let l: i32 = it.next().unwrap().parse().unwrap();
        let mut a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();

        let initial = a;
        let mut max = a;

        loop {
            a = (a+b) % l;

            if a > max {
                max = a;
            }

            if a == initial {
                break;
            }
        }
        
        result.push(max);
    }

    for val in result {
        println!("{val}");
    }
}
