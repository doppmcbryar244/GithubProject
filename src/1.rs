// Generating a random Rust code snippet using the rand crate
use rand::Rng;

let mut rng = rand::thread_rng();
let random_number: u8 = rng.gen_range(1, 10);

match random_number {
    1 => println!("First"),
    2 => println!("Second"),
    3 => println!("Third"),
    4 => println!("Fourth"),
    5 => println!("Fifth"),
    6 => println!("Sixth"),
    7 => println!("Seventh"),
    8 => println!("Eighth"),
    9 => println!("Ninth"),
    10 => println!("Tenth"),
    _ => panic!("Invalid number!"),
}
