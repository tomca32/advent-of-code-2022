use std::{fs::File, io::{BufReader, BufRead}};

use slice_deque::SliceDeque;

enum CraneType {
    SingleContainerMove,
    MultipleContainerMove,
}

#[derive(Debug)]
struct ContainerStack {
    stack: SliceDeque<char>
}


impl ContainerStack {
    fn new() -> Self {
        Self { stack: SliceDeque::new() }
    }

    fn put_containers(&mut self, containers: &[char]) -> &[char]{
        for &c in containers {
            self.stack.push_back(c);
        }
        self.slice()
    }

    fn put_multiple_containers(&mut self, containers: &[char]) -> &[char] {
        for &c in containers.iter().rev() {
            self.stack.push_back(c);
        }
        self.slice()
    }

    fn put_container_on_bottom(&mut self, container: char) {
        self.stack.push_front(container);
    }

    fn take_containers(&mut self, number: u32) -> Vec<char> {
        let mut ret = vec![];
        for _ in 0..number {
            ret.push(self.stack.pop_back().expect("Stack empty, cannot pop."));
        }
        ret
    }

    fn slice<'a>(&'a self) -> &'a [char] {
        &self.stack
    }

    fn top(&self) -> char {
        *self.stack.back().unwrap()
    }
}

#[test]
fn test_container_stack() {
    let mut s = ContainerStack::new();
    s.put_container_on_bottom('G');
    s.put_container_on_bottom('B');
    s.put_container_on_bottom('D');
    s.put_container_on_bottom('C');
    s.put_container_on_bottom('P');
    s.put_container_on_bottom('R');
    assert_eq!(s.slice(), &['R', 'P', 'C', 'D', 'B', 'G']);
    assert_eq!(s.take_containers(3), &['G', 'B', 'D']);
    assert_eq!(s.slice(), &['R', 'P', 'C']);
    assert_eq!(s.put_containers(&['G', 'B', 'D']), &['R', 'P', 'C', 'G', 'B', 'D']);

}

fn parse_stackline(line: &str, mut stacks: Vec<ContainerStack>) -> Vec<ContainerStack> {
    for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 {
            let stack_index = i / 4;
            if stacks.len() <= stack_index {
                for _ in 0..=stack_index - stacks.len() {
                    stacks.push(ContainerStack::new());
                }
            }

            if c != ' ' {
                stacks[stack_index].put_container_on_bottom(c);
            }
        }
    }

    stacks
}

#[test]
fn parse_stackline_test() {
    let mut stacks = parse_stackline("[G]     [P] [C] [F] [G] [T]", vec![]);
    stacks = parse_stackline("[B]     [J] [D] [P] [V] [F] [F]", stacks);
    assert_eq!(stacks[0].slice(), &['B', 'G']);
    assert_eq!(stacks[1].slice(), &[]);
    assert_eq!(stacks[2].slice(), &['J', 'P']);
    assert_eq!(stacks[3].slice(), &['D', 'C']);
}

fn parse_move_line(line: &str, stacks: &mut [ContainerStack], crane_type: &CraneType) {
    let split_line = line.split(' ').collect::<Vec<&str>>();
    let amount = split_line[1].parse::<u32>().expect("Failed parsing number from input");
    let from = split_line[3].parse::<usize>().expect("Failed parsing number from input") - 1;
    let to = split_line[5].parse::<usize>().expect("Failed parsing number from input") - 1;
    let target_containers = stacks[from].take_containers(amount);
    match crane_type {
        CraneType::SingleContainerMove => stacks[to].put_containers(&target_containers),
        CraneType::MultipleContainerMove => stacks[to].put_multiple_containers(&target_containers),
    };
}

#[test]
fn parse_move_line_test() {
    let mut stacks = parse_stackline("[G]     [P] [C] [F] [G] [T]", vec![]);
    stacks = parse_stackline("[B]     [J] [D] [P] [V] [F] [F]", stacks);
    parse_move_line("move 1 from 1 to 2", &mut stacks, &CraneType::SingleContainerMove);
    assert_eq!(stacks[0].slice(), &['B']);
    assert_eq!(stacks[1].slice(), &['G']);
    parse_move_line("move 1 from 2 to 3", &mut stacks, &CraneType::SingleContainerMove);
    assert_eq!(stacks[1].slice(), &[]);
    assert_eq!(stacks[2].slice(), &['J', 'P', 'G']);
    parse_move_line("move 3 from 3 to 2", &mut stacks, &CraneType::MultipleContainerMove);
    assert_eq!(stacks[1].slice(), &['J', 'P', 'G']);
}

fn apply_crane(reader: BufReader<File>, crane_type: CraneType) -> Vec<ContainerStack> {
    let mut stacks: Vec<ContainerStack> = vec![];
    for line in reader.lines() {
        let l = line.expect("Error reading line");
        if l.trim().is_empty() {
            continue;
        }
        if l.starts_with("move") {
            parse_move_line(&l, &mut stacks, &crane_type);
            continue;
        }
        if l.trim().as_bytes()[0].is_ascii_digit() {
            continue;
        }
        stacks = parse_stackline(&l, stacks);
    }
    stacks
}

fn first_part() -> Vec<ContainerStack> {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);
    apply_crane(reader, CraneType::SingleContainerMove)
}

fn second_part() -> Vec<ContainerStack> {
    let file = File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);
    apply_crane(reader, CraneType::MultipleContainerMove)
}

fn main() {
    println!("{}", first_part().iter().map(|s| s.top()).collect::<String>());
    println!("{}", second_part().iter().map(|s| s.top()).collect::<String>());
}
