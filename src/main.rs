use std::{cmp::Ordering, intrinsics::mir::Len, io};

use rand::Rng;

fn main() {
    const test_const: u32 = 2 * 1;
    let seq = 213;

    {
        // let test_const = "same?"; err
        // const test_const = 1231; err
        let seq = 3213;
    }

    let immustr = "132";
    let immustr = immustr.len();

    let parsed = "123123".parse::<u32>().expect("err");
    let parsed = "123123".parse::<i32>().expect("err");

    let tup: (u32, i32, f32) = (1, 1, 1.0);
    let (bu, n, hae) = tup;

    let accidx = tup.0;

    let thisislist = ["typeorval";5];
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
