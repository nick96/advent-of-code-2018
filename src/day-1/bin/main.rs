// Day 1: Chronal Calibration
//
// Determine the resulting frequency after all changes in frequency
// have been applied, starting from a frequency of zero.
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Day 1");
    println!("=====");

    part_1();
}

fn part_1() {
    println!("Part 1");
    println!("------");
    let filename = "input_part1.txt";
    let fh = File::open(filename).expect("file not found");
    let freq = determine_freq(fh);
    println!("Frequency: {}", freq.unwrap());
}

// determine_freq solves part 1.
fn determine_freq<R: Read>(reader: R) -> Result<i32, String> {
    let mut freq = 0;
    for line in BufReader::new(reader).lines() {
        freq += line.unwrap().parse::<i32>().unwrap()
    }
    Ok(freq)
}

#[test]
fn example1() {
    use std::io::Cursor;
    let deltas = vec!["+1", "-2", "+3", "+1"];
    let input = deltas.join("\n");
    let buf = Cursor::new(input.as_bytes());
    let freq = determine_freq(buf).unwrap();
    assert_eq!(freq, 3);
}

#[test]
fn example2() {
    use std::io::Cursor;
    let deltas = vec!["+1", "+1", "+1"];
    let input = deltas.join("\n");
    let buf = Cursor::new(input.as_bytes());
    let freq = determine_freq(buf).unwrap();
    assert_eq!(freq, 3);
}

#[test]
fn example3() {
    use std::io::Cursor;
    let deltas = vec!["+1", "+1", "-2"];
    let input = deltas.join("\n");
    let buf = Cursor::new(input.as_bytes());
    let freq = determine_freq(buf).unwrap();
    assert_eq!(freq, 0);
}

#[test]
fn example4() {
    use std::io::Cursor;
    let deltas = vec!["-1", "-2", "-3"];
    let input = deltas.join("\n");
    let buf = Cursor::new(input.as_bytes());
    let freq = determine_freq(buf).unwrap();
    assert_eq!(freq, -6);
}
