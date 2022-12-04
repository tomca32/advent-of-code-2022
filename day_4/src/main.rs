use std::{io::{BufReader, BufRead}, fs::File};

struct Pair {
    first: SectionRange,
    second: SectionRange,
}

impl FromIterator<SectionRange> for Pair {
    fn from_iter<T: IntoIterator<Item = SectionRange>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();
        let first = into_iter.next().expect("Cannot construct Pair from an empty iterator.");
        let second = into_iter.next().expect("Cannot construct Pair from an empty iterator.");
        Self {first, second}
    }
}

struct SectionRange {
    from: u32,
    to: u32,
}

impl FromIterator<u32> for SectionRange {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();
        let from = into_iter.next().expect("Cannot construct SectionRange from an empty iterator");
        let to = into_iter.next().expect("Cannot construct SectionRange from an empty iterator");
        Self {from, to}
    }
}

fn is_fully_contained_in(needle: &SectionRange, haystack: &SectionRange) -> bool {
    needle.from >= haystack.from && needle.to <= haystack.to
}

fn are_overlapping(a: &SectionRange, b: &SectionRange) -> bool {
    a.from >= b.from && a.from <= b.to || b.from >= a.from && b.from <= a.to
}

#[test]
fn is_fully_contained_in_test() {
    assert!(is_fully_contained_in(&SectionRange {from: 2, to: 4}, &SectionRange { from: 1, to: 8 }));
    assert!(!is_fully_contained_in(&SectionRange {from: 2, to: 4}, &SectionRange { from: 6, to: 8 }));
}

#[test]
fn are_overlapping_test() {
    assert!(are_overlapping(&SectionRange {from: 2, to: 4}, &SectionRange { from: 1, to: 5 }));
    assert!(are_overlapping(&SectionRange {from: 1, to: 5}, &SectionRange { from: 2, to: 4 }));
    assert!(are_overlapping(&SectionRange {from: 1, to: 3}, &SectionRange { from: 2, to: 4 }));
    assert!(are_overlapping(&SectionRange {from: 3, to: 7}, &SectionRange { from: 2, to: 4 }));
    assert!(are_overlapping(&SectionRange {from: 2, to: 4}, &SectionRange { from: 3, to: 7 }));
}

fn first_part() {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);
    let result = reader.lines().fold(0u32, |acc, line| {
        let pair = line.unwrap().split(',').map(|section_range| {
            section_range.split('-').map(|section_part| section_part.parse::<u32>().unwrap()).collect::<SectionRange>()
        }).collect::<Pair>();
        if is_fully_contained_in(&pair.first, &pair.second) || is_fully_contained_in(&pair.second, &pair.first) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{result} pairs have fully overlaping sections");
}

fn second_part() {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);
    let result = reader.lines().fold(0u32, |acc, line| {
        let pair = line.unwrap().split(',').map(|section_range| {
            section_range.split('-').map(|section_part| section_part.parse::<u32>().unwrap()).collect::<SectionRange>()
        }).collect::<Pair>();
        if are_overlapping(&pair.first, &pair.second) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{result} pairs have overlaping sections");
}

fn main() {
    first_part();
    second_part();
}
