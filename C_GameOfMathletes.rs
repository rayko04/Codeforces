use std::io::{self, Read};
use std::collections::HashMap;

fn sol(res: &mut Vec<i32>, vec: &[i32], k: i32) {
    let mut freq = HashMap::new();
    
    for &val in vec {
        *freq.entry(val).or_insert(0) += 1;
    }

    let mut score = 0;
    for (&x, &count_x) in &freq {
        let y = k-x;

        if x < y {
            if let Some(&count_y) = freq.get(&y) {
                score += count_x.min(count_y);
            }
        }
        else if x == y {
            score += count_x/2;
        }
    }
    res.push(score);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..t {
        let n: i32 = it.next().unwrap().parse().unwrap();
        let k: i32 = it.next().unwrap().parse().unwrap();
        
        let mut vec: Vec<i32> = Vec::new();
        for _ in 0..n {
            vec.push(it.next().unwrap().parse().unwrap());
        }

        sol(&mut res, &vec, k);
    }

    for r in res {
        println!("{}", r);
    }
}
