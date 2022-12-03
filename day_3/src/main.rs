use std::{fs::{File, self}, io::{BufReader, BufRead}, collections::HashMap};

fn char_to_priority(c: char) -> u32 {
    if !c.is_ascii() {
        panic!("Invalid non-ASCII character: {c}");
    }
    if c.is_uppercase() {
        (c as u8 - 38) as u32
    } else if c.is_lowercase() {
        (c as u8 - 96) as u32
    } else {
        panic!("Character is non alphabetic: {c}");
    }
}

#[test]
fn char_to_priority_test() {
    assert_eq!(char_to_priority('p'), 16);
    assert_eq!(char_to_priority('L'), 38);
    assert_eq!(char_to_priority('P'), 42);
    assert_eq!(char_to_priority('v'), 22);
    assert_eq!(char_to_priority('t'), 20);
    assert_eq!(char_to_priority('s'), 19);
}

fn find_shared_character(strings: &[&str]) -> Option<char> {
    let mut chars = HashMap::new();

    for (i, string) in strings.into_iter().enumerate() {
        if i == strings.len() -1 {
            for c in string.chars() {
                if let Some(existing) = chars.get(&c) {
                    if i - 1 == *existing {return Some(c)}
                }
            }
        } else {
            for c in string.chars() {
                match chars.get(&c) {
                    Some(existing) => if i == 0 || i - 1 == *existing {chars.insert(c, i);},
                    None => if i == 0 {chars.insert(c, i);},
                }
            }
        }
    }
    None
}

#[test]
fn find_shared_character_test() {
    assert_eq!(find_shared_character(&["vJrwpWtwJgWr", "hcsFMMfFFhFp"]), Some('p'));
    assert_eq!(find_shared_character(&["vJrwWtwJgWr", "hcsFMMfFFhF"]), None);
    assert_eq!(find_shared_character(&["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"]), Some('L'));
    assert_eq!(find_shared_character(&["jqHRNqRjqzjGDG", "rsFMfFZSrrFZsS"]), None);
    assert_eq!(find_shared_character(&["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"]), Some('r'));
}

fn first_part() {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);
    let priority_sum = reader.lines().fold(0u32, |acc, line| {
        let line = line.unwrap();
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let shared_character = find_shared_character(&[first_half, second_half]);
        acc + char_to_priority(shared_character.unwrap())
    });

    println!("Sum of all priorities is: {priority_sum}");
}

fn second_part() {
    let contents = fs::read_to_string("./src/input.txt").expect("Failed reading file");
    let backpacks = contents.split('\n').collect::<Vec<_>>();
    let badge_sum = backpacks.chunks(3).fold(0u32, |acc, chunk| {
        let badge = find_shared_character(chunk).expect("Failed to find shared character in chunk");
        acc + char_to_priority(badge)
    });

    println!("Sum of priorities of badges is: {badge_sum}");
}

fn main() {
    first_part();
    second_part();
}
