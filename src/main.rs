use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let rannum: u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Input: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");
        println!("This is interpolation, input: {input}");
        let parsed = match input.trim().parse::<u32>() {
            Err(_) => continue,
            Ok(n) => n,
        };

        match parsed.cmp(&rannum) {
            Ordering::Equal => {
                println!("T");
                break;
            }
            Ordering::Less => println!("F"),
            Ordering::Greater => println!("F"),
        }
    }
}
