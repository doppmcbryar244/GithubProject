fn main() {
    let mut count = 0;
    let mut sum = 0;

    for i in 1..=100 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
            count += 1;
        }
    }

    println!("The sum of the multiples of 3 or 5 below 100 is {}", sum);
}
