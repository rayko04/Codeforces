use std::io::{self, Read};

fn main() {
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut scores: Vec<i32> = Vec::with_capacity(3);

    for _ in 0..3 {
        scores.push(it.next().unwrap().parse().unwrap());
    }

    scores.sort();

    if scores[2] - scores[0] > 9 {
        println!("check again");
        return;
    }
    
    println!("final {}", scores[1]);
}
