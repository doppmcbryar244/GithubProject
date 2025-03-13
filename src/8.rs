use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    let mut file = File::open("data.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}
