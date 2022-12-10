use std::{fs, io::{BufReader, BufRead}};

type TreeMap = Vec<Vec<u32>>;

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn new_position(&self, x: usize, y: usize, map: &TreeMap) -> Option<(usize, usize)> {
        let coords = match self {
            Direction::Up => (x, y.checked_sub(1)?),
            Direction::Right => (x.checked_add(1)?, y),
            Direction::Down => (x, y.checked_add(1)?),
            Direction::Left => (x.checked_sub(1)?, y),
        };

        if let Some(_) = map.get(coords.1)?.get(coords.0) {
            Some(coords)
        } else {
            None
        }
    }

    fn is_tree_visible(&self, x: usize, y: usize, map: &TreeMap) -> bool {
        let mut _x = x;
        let mut _y = y;
        let tree = get_tree((x, y), map);

        while let Some(new_coords) = self.new_position(_x, _y, map) {
            let new_tree = get_tree(new_coords, map);
            if new_tree >= tree {
                return false;
            }
            _x = new_coords.0;
            _y = new_coords.1;
        }

        true
    }
}

fn get_tree(coords: (usize, usize), map: &TreeMap) -> u32 {
    map[coords.1][coords.0]
}

const DIRECTION: [Direction; 4] = [Direction::Up, Direction::Right, Direction:: Down, Direction::Left];

fn is_edge(x: usize, y: usize, map: &TreeMap) -> bool {
    x == 0 || y == 0 || y == map.len() || y == map.get(y).expect("Y is out of bounds").len()
}

fn is_tree_visible(x: usize, y: usize, map: &TreeMap) -> bool {
    if is_edge(x, y, map) {
        return true
    }

    for d in &DIRECTION {
        if d.is_tree_visible(x, y, map) {
            return true;
        }
    }

    false
}

fn tree_scenic_score(x: usize, y: usize, map: &TreeMap) -> u32 {
    if is_edge(x, y, map) {
        return 0;
    }

    let tree = get_tree((x, y), map);

    let direction_scores = &DIRECTION.map(|d| {
        let mut _x = x;
        let mut _y = y;

        let mut direction_score = 0;

        while let Some(new_coords) = d.new_position(_x, _y, map) {
            let new_tree = get_tree(new_coords, map);
            direction_score += 1;
            if new_tree >= tree {
                break;
            }
            _x = new_coords.0;
            _y = new_coords.1;
        }
        direction_score
    });

    direction_scores.iter().product()
}

fn parse_map() -> TreeMap {
    let map = TreeMap::new();
    let file = fs::File::open("./src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| {
        l.unwrap().chars().map(|height| height as u32).collect::<Vec<_>>()
    }).collect()

}

fn first_part(map: &TreeMap) -> u32 {
    let mut number_of_visible_trees = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if is_tree_visible(x, y, map) {
                number_of_visible_trees += 1;
            }
        }
    }
    println!("There are {number_of_visible_trees} visible trees.");
    number_of_visible_trees
}

fn second_part(map: &TreeMap) {
    let mut max_scenic_score = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let score = tree_scenic_score(x, y, map);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    println!("Max Scenic score for all trees is: {}", max_scenic_score);
}

fn main() {
    let map = parse_map();
    first_part(&map);
    second_part(&map);
}


fn test_vector() -> TreeMap {
    vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ]
}

#[test]
fn is_tree_visible_test() {
    let map = test_vector();
    assert!(is_tree_visible(0, 0, &map));
    assert!(is_tree_visible(1, 0, &map));
    assert!(is_tree_visible(0, 1, &map));
    assert!(is_tree_visible(4, 4, &map));
    assert!(is_tree_visible(2, 4, &map));
    assert!(is_tree_visible(4, 2, &map));
    assert!(is_tree_visible(1, 2, &map));
    assert!(is_tree_visible(2, 1, &map));
    assert!(is_tree_visible(1, 1, &map));
    assert!(!is_tree_visible(3, 1, &map));
    assert!(!is_tree_visible(1, 3, &map));
    assert!(!is_tree_visible(3, 3, &map));
    assert!(!is_tree_visible(2, 2, &map));
}

#[test]
fn first_part_test() {
    let map = test_vector();
    assert_eq!(first_part(&map), 21);
}

#[test]
fn tree_scenic_score_test() {
    let map = test_vector();
    assert_eq!(tree_scenic_score(2, 1, &map), 4);
    assert_eq!(tree_scenic_score(2, 3, &map), 8);
}