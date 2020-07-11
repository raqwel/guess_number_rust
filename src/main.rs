
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_num = rand::thread_rng().gen_range(1, 100);

    println!("{}", secret_num);

    loop {
        println!("Guess the number!");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("failed to load");

        let input : u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) =>continue,
        };


        println!("You guess {}", input);

        match input.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("Tada!");
                break;
            },
            Ordering::Greater => println!("too big"),
        }
    }

}
