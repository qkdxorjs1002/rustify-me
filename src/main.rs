use std::{cmp::Ordering, intrinsics::mir::Len, io};

use rand::Rng;

fn main() {
    const test_const: u32 = 2 * 1;
    let seq = 213;

    {
        // let test_const = "same?"
        // const test_const = 1231;
        let seq = 3213;
    }

    let immustr = "132";
    let immustr = immustr.len();
}

fn main2() {
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
