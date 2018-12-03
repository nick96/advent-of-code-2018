// Day 2: Inventory Management System
//
// Determine the checksum of the list of box IDs. The checksum is the
// product of the number of boxes with IDs containing exactly two of a
// given letter and IDs containing exactly three of a given letter.
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Day 2");
    println!("=====");

    part_1();
}

fn part_1() {
    println!("Part 1");
    println!("------");

    let filename = "day2_input.txt";
    let fh = File::open(filename).expect("File not found");
    let checksum = determine_checksum(fh);
    println!("Checksum: {}", checksum);
}

fn determine_checksum<R: Read>(reader: R) -> i32 {
    let mut two_count = 0;
    let mut three_count = 0;

    for line in BufReader::new(reader).lines() {
        let id = line.unwrap();
        two_count += contains_double(&id) as i32;
        three_count += contains_triple(&id) as i32;
    }

    return two_count * three_count;
}

fn contains_double(id: &String) -> bool {
    contains_n_reps(id, 2)
}

fn contains_triple(id: &String) -> bool {
    contains_n_reps(id, 3)
}

fn contains_n_reps(s: &String, n: i32) -> bool {
    let counts = letter_counts(s);
    for count in counts.values() {
        if *count == n {
            return true;
        }
    }
    false
}

fn letter_counts(s: &String) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for c in s.chars() {
        let count = *counts.entry(c).or_insert(0);
        counts.insert(c, count + 1);
    }

    counts
}

#[cfg(test)]
mod day2tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn part1_example1() {
        let ids = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        let input = ids.join("\n");
        let buf = Cursor::new(input.as_bytes());
        let checksum = determine_checksum(buf);
        assert_eq!(checksum, 12);
    }

    #[test]
    fn contains_double_test() {
        assert!(!contains_double(&"abcdef".to_string()));
        assert!(contains_double(&"bababc".to_string()));
        assert!(contains_double(&"abbcde".to_string()));
        assert!(!contains_double(&"abcccd".to_string()));
        assert!(contains_double(&"aabcdd".to_string()));
        assert!(contains_double(&"abcdee".to_string()));
        assert!(!contains_double(&"ababab".to_string()));
    }

    #[test]
    fn contains_triple_test() {
        assert!(!contains_triple(&"abcdef".to_string()));
        assert!(contains_triple(&"bababc".to_string()));
        assert!(!contains_triple(&"abbcde".to_string()));
        assert!(contains_triple(&"abcccd".to_string()));
        assert!(!contains_triple(&"aabcdd".to_string()));
        assert!(!contains_triple(&"abcdee".to_string()));
        assert!(contains_triple(&"ababab".to_string()));
    }
}
