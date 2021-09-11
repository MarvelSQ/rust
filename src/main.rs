use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let n = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess.");
        let mut str = String::new();
        let my = String::from(&str);
        std::io::stdin().read_line(&mut str).expect("failed");
        let str: u32 = match str.trim().parse() {
            Ok(num) => num,
            Err(err) => {
              println!("error: {}", err);
              println!("number required");
              continue
            },
        };

        match str.cmp(&n) {
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("same");
                break;
            }
            Ordering::Greater => println!("big"),
        }
    }
}
