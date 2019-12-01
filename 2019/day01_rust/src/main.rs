use std::io::BufReader;
use std::fs::File;
use std::io::prelude::BufRead;

fn main() {
    let path = std::path::Path::new("./input1.txt");
    let f = File::open(path).expect("Couldn't open file");
    let f = BufReader::new(f);

    let mut sum: u32 = 0;
    for line in f.lines() {
        let n: u32 = line.unwrap().trim().parse()
            .expect("Failed to parse");
        let n: u32 = n / 3;
        let n: u32 = n - 2;
        sum = sum + n
    }

    println!("{}", sum);
}
