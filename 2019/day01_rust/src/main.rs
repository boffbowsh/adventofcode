use std::io::BufReader;
use std::fs::File;
use std::io::prelude::BufRead;

fn main() {
    let path = std::path::Path::new("./input1.txt");
    let f = File::open(path).expect("Couldn't open file");
    let f = BufReader::new(f);

    let mut fuel = 0;
    for line in f.lines() {
        let n: u32 = line.unwrap().trim().parse()
            .expect("Failed to parse");
        fuel = fuel + fuel_for_mass(n);
    }

    println!("With fuel mass: {}", fuel);
}

fn fuel_for_mass(mass: u32) -> u32 {
    let n = mass / 3;

    if n < 2 {
        0
    } else {
        let fuel = n - 2;
        fuel + fuel_for_mass(fuel)
    }
}
