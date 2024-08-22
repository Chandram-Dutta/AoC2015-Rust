use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("day_01.txt")?;

    let start = Instant::now();
    part01(&content);
    let duration = start.elapsed();
    println!("Execution Time 1: {:?}", duration);

    let start = Instant::now();
    part02(&content);
    let duration = start.elapsed();
    println!("Execution Time 2: {:?}", duration);

    Ok(())
}

fn part01(content: &str) {
    let mut floor = 0;
    for c in content.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    println!("Answer 1: {}", floor);
}

fn part02(content: &str) {
    let mut floor = 0;
    for (index, c) in content.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            println!("Answer 2: {}", index + 1);
            break;
        }
    }
}
