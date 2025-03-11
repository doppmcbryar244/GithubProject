fn main() {
    let x = f64::rand(0..1);
    if x < 0.5 {
        println!("Hello World!");
    } else {
        println!("Goodbye World!");
    }
}
