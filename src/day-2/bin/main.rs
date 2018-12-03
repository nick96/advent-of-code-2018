// Day 2: Inventory Management System
//
// Determine the checksum of the list of box IDs. The checksum is the
// product of the number of boxes with IDs containing exactly two of a
// given letter and IDs containing exactly three of a given letter.
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Day 2");
    println!("=====");

    part_1();
}

fn part_1() {
    println!("Part 1");
    println!("------");

    let filename = "input.txt";
    let fh = File::open(filename).expect("File not found");
    let checksum = determine_checksum(fh);
    println!("Checksum: {}", checksum);
}

fn determine_checksum<R: Read>(_reader: R) -> i32 {
    return 0;
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
}
