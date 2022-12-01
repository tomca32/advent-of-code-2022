use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Failed reading file");
    let split_contents = contents.split("\n\n").collect::<Vec<&str>>();
    let mut grouped_calories = split_contents.into_iter()
        .map(|e| e.split("\n").fold(0u32, |acc, item| acc + item.parse::<u32>().expect("Failed parsing string")))
        .collect::<Vec<u32>>();
    grouped_calories.sort_by(|a, b| b.cmp(a));
    let max_calories = grouped_calories[0];
    let top_3_calories: u32 = grouped_calories[..3].iter().sum();
    println!("Top elf calories: {:?}", max_calories);
    println!("Top 3 elves calories: {:?}", top_3_calories);
}
