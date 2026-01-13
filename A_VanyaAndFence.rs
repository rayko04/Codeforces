use std::io::{self, Read};

fn main() {
    
    let mut guess = String::new();

    io::stdin()
        .read_to_string(&mut guess)
        .unwrap();
    
    let mut it = guess.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let h: i32 = it.next().unwrap().parse().unwrap(); 

    let mut a:Vec<i32> = Vec::with_capacity(n);
   
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    let mut count = 0;

    for element in &a {
        if *element > h {
            count+=2;
        }
        else {
            count+=1;
        }
    }

    println!("{count}");
}
