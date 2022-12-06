use std::{collections::HashMap, fs};
mod queue;

fn all_chars_are_different(s: &str) -> bool {
    let mut existing_chars = HashMap::new();
    for c in s.chars() {
        if existing_chars.contains_key(&c) {
            return false;
        }
        existing_chars.insert(c, true);
    }
    true
}

#[test]
fn all_chars_are_different_test() {
    assert!(all_chars_are_different("asdf"));
    assert!(all_chars_are_different("mnbv"));
    assert!(!all_chars_are_different("asda"));
    assert!(!all_chars_are_different("asds"));
    assert!(!all_chars_are_different("amvm"));
}

fn find_first_marker_last_character_index(input: &str, buffer_size: usize) -> Option<usize> {
    let mut queue = queue::CircularQueue::with_capacity(buffer_size);

    for (i, c) in input.chars().enumerate() {
        queue.push(c);
        if queue.is_full() && all_chars_are_different(&queue.iter().collect::<String>()) {
            return Some(i + 1);
        }
    }
    None
}

#[test]
fn find_first_marker_last_character_index_test() {
    assert_eq!(find_first_marker_last_character_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    assert_eq!(find_first_marker_last_character_index("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    assert_eq!(find_first_marker_last_character_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
    assert_eq!(find_first_marker_last_character_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));

    assert_eq!(find_first_marker_last_character_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
    assert_eq!(find_first_marker_last_character_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
    assert_eq!(find_first_marker_last_character_index("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
    assert_eq!(find_first_marker_last_character_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), Some(29));
    assert_eq!(find_first_marker_last_character_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some(26));
}

fn first_part() {
    let input = fs::read_to_string("./src/input.txt").expect("Failed reading file.");
    let last_character_of_ordinality = find_first_marker_last_character_index(&input, 4).expect("No signal found");
    println!("Signal detected after {} characters.", last_character_of_ordinality);
}

fn second_part() {
    let input = fs::read_to_string("./src/input.txt").expect("Failed reading file.");
    let last_character_of_ordinality = find_first_marker_last_character_index(&input, 14).expect("No message found");
    println!("Message detected after {} characters.", last_character_of_ordinality);
}

fn main() {
    first_part();
    second_part();
}
