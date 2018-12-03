// Day 1: Chronal Calibration
//
// Determine the resulting frequency after all changes in frequency
// have been applied, starting from a frequency of zero. Then
// determine the first frequency repeated by the given list of
// frequenct deltas.
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Day 1");
    println!("=====");

    part_1();

    println!();

    part_2();
}

fn part_1() {
    println!("Part 1");
    println!("------");
    let filename = "input.txt";
    let fh = File::open(filename).expect("file not found");
    let freq = determine_freq(fh);
    println!("Frequency: {}", freq);
}

fn part_2() {
    println!("Part 2");
    println!("------");

    let filename = "input.txt";
    let fh = File::open(filename).expect("file not found");
    let repeated_freq = find_repeated_freq(fh);
    println!("Repeated frequency: {}", repeated_freq);
}

// determine_freq finds the frequency after all the chanages in reader
// have been applied.
fn determine_freq<R: Read>(reader: R) -> i32 {
    let mut freq = 0;
    for line in BufReader::new(reader).lines() {
        freq += line.unwrap().parse::<i32>().unwrap()
    }
    freq
}

// find_repeated_freq find the first frequency repeated twice by the
// changes in reader.
fn find_repeated_freq<R: Read + Seek>(mut reader: R) -> i32 {
    let mut buf = String::new();
    reader.read_to_string(&mut buf).ok();

    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(freq);
    loop {
        for line in buf.split("\n") {
            if line.trim() != "" {
                freq += line.parse::<i32>().expect("Could not parse line");
                if seen.contains(&freq) {
                    return freq;
                }
                seen.insert(freq);
            }
        }
    }
}

#[cfg(test)]
mod day1tests {
    use super::*;

    #[test]
    fn part1_example1() {
        use std::io::Cursor;
        let deltas = vec!["+1", "-2", "+3", "+1"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = determine_freq(buf);
        assert_eq!(freq, 3);
    }

    #[test]
    fn part1_example2() {
        use std::io::Cursor;
        let deltas = vec!["+1", "+1", "+1"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = determine_freq(buf);
        assert_eq!(freq, 3);
    }

    #[test]
    fn part1_example3() {
        use std::io::Cursor;
        let deltas = vec!["+1", "+1", "-2"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = determine_freq(buf);
        assert_eq!(freq, 0);
    }

    #[test]
    fn part1_example4() {
        use std::io::Cursor;
        let deltas = vec!["-1", "-2", "-3"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = determine_freq(buf);
        assert_eq!(freq, -6);
    }

    #[test]
    fn part1_challenge() {
        let filename = "input.txt";
        let fh = File::open(filename).expect("file not found");
        let freq = determine_freq(fh);
        assert_eq!(freq, 510);
    }

    #[test]
    fn part2_example1() {
        use std::io::Cursor;
        let deltas = vec!["+1", "-2", "+3", "+1", "+1", "-2"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = find_repeated_freq(buf);
        assert_eq!(freq, 2);
    }

    #[test]
    fn part2_example2() {
        use std::io::Cursor;
        let deltas = vec!["+1", "-1"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = find_repeated_freq(buf);
        assert_eq!(freq, 0);
    }

    #[test]
    fn part2_example3() {
        use std::io::Cursor;
        let deltas = vec!["+3", "+3", "+4", "-2", "-4"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = find_repeated_freq(buf);
        assert_eq!(freq, 10);
    }

    #[test]
    fn part2_example4() {
        use std::io::Cursor;
        let deltas = vec!["-6", "+3", "+8", "+5", "-6"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = find_repeated_freq(buf);
        assert_eq!(freq, 5);
    }

    #[test]
    fn part2_example5() {
        use std::io::Cursor;
        let deltas = vec!["+7", "+7", "-2", "-7", "-4"];
        let input = deltas.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let freq = find_repeated_freq(buf);
        assert_eq!(freq, 14);
    }

    #[test]
    fn part2_challenge() {
        let filename = "input.txt";
        let fh = File::open(filename).expect("file not found");
        let freq = find_repeated_freq(fh);
        assert_eq!(freq, 69074);
    }

}
