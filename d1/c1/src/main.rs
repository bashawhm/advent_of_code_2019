use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn mass_to_fuel(mass: f64) -> f64 {
    return (mass / 3.0).floor() - 2.0;
}


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let numbers: Vec<i32> = 
        contents.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sum = 0;
    for num in numbers {
        let res = mass_to_fuel(num as f64);
        sum += res as i32;
    }

    println!("{}", sum);

    Ok(())
}
