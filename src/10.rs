fn main() {
    let mut list = Vec::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    for element in &list {
        println!("{}", element);
    }
}
