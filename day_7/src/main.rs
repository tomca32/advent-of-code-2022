use core::panic;
use std::{io::{BufReader, BufRead}, fs::{self, File}, collections::{HashMap, HashSet}, path::{Path, PathBuf}};
use path_absolutize::*;

#[derive(Debug, PartialEq)]
enum Command {
    List,
    ChangeDirectory(String),
}

type FileArena = HashMap<String, Node>;

const TOTAL_SPACE:usize = 70000000;

impl Command {
    fn from_line(line: &str) -> Option<Self> {
        if !Self::is_command(line) {
            return None;
        }
        let mut split = line.split(' ');
        split.next();
        let command = split.next().unwrap_or_else(|| panic!("Cannot parse command from line: {}", &line));

        match command {
            "ls" => Some(Command::List),
            "cd" => {
                let path = split.next().unwrap_or_else(|| panic!("Cannot parse path from line: {}", &line));
                Some(Command::ChangeDirectory(String::from(path)))
            },
            _ => panic!("Invalid command {} in line: {}", &command, &line)
        }
    }

    fn is_command(line: &str) -> bool {
        line.starts_with("$")
    }

    fn change_directory(cd: &str, current_directory: &mut PathBuf) {
        *current_directory = current_directory.join(cd).absolutize().expect("Failed getting parent path").to_path_buf();
    }

    fn execute_command(&self, current_directory: &mut PathBuf) {
        match self {
            Command::List => (),
            Command::ChangeDirectory(cd) => Self::change_directory(cd, current_directory),
        }
    }
}

struct FileProperties {
    name: String,
    size: usize,
}

struct DirectoryProperties {
    name: String,
    children: HashSet<String>
}

enum Node {
    File(FileProperties),
    Directory(DirectoryProperties),
}

impl Node {
    fn get_size(&self, file_arena: &FileArena) -> usize {
        match self {
            Node::File(f) => f.size,
            Node::Directory(d) => d.children.iter().fold(0, |acc, e| acc + file_arena.get(e).unwrap().get_size(file_arena)),
        }
    }
}

fn process_directory_list(line: &str, current_directory: &Path, file_arena: &mut FileArena) {
    let mut line_iter = line.split(' ');
    let size_or_dir = line_iter.next().expect("failed getting dir keyword or size of file");
    let name = line_iter.next().expect("Failed getting file or directory name.");
    let joined_path = current_directory.join(name);
    let target_path = joined_path.to_str().unwrap();
    if !file_arena.contains_key(target_path) {
        let new_node = match size_or_dir == "dir" {
            true => Node::Directory(DirectoryProperties{ name: String::from(name), children: HashSet::new() }),
            false => Node::File(FileProperties{ name: String::from(name), size: size_or_dir.parse::<usize>().unwrap_or_else(|_| panic!("Failed parsing size: {}", size_or_dir)) }),
        };
        file_arena.insert(target_path.to_owned(), new_node);
        let current_dir_node = file_arena.get_mut(current_directory.to_str().unwrap()).unwrap();
        match current_dir_node {
            Node::File(_) => panic!("Current directory is somehow a file"),
            Node::Directory(ref mut props) => props.children.insert(target_path.to_owned()),
        };
    }
}

fn used_space(file_arena: &FileArena) -> usize {
    let root = file_arena.get("/").unwrap();
    root.get_size(file_arena)
}

fn find_subdirectories<'a>(dir: &'a Node, file_arena: &'a FileArena, min_size: usize) -> Vec<&'a Node> {
    let mut ret = vec![];
    if let Node::Directory(dir_props) = dir {
        if dir.get_size(file_arena) >= min_size {
            ret.push(dir);
            for subdir_name in dir_props.children.iter() {
                let node = file_arena.get(subdir_name).unwrap();
                ret = [ret, find_subdirectories(node, file_arena, min_size)].concat();
            }
        }
    }
    ret
}

fn first_part(file_arena: &FileArena) {
    println!("Total size: {}", file_arena.get("/").unwrap().get_size(file_arena));
    let dirs_smaller_than_100000_size = file_arena.iter()
        .filter(|(_, value)| matches!(value, Node::Directory(_)) && value.get_size(file_arena) < 100000)
        .fold(0, |acc, (_, dir)| acc + dir.get_size(file_arena));

    println!("Total of dirs smaller than 100000: {}", dirs_smaller_than_100000_size);
}

fn second_part(file_arena: &FileArena) {
    let space_used = used_space(file_arena);
    let space_available = TOTAL_SPACE - space_used;
    let space_required = 30000000 - space_available;

    println!("Disk size is: {}", TOTAL_SPACE);
    println!("Space used is: {}", space_used);
    println!("Space required is: {}", space_required);
    let root = file_arena.get("/").unwrap();
    if let Node::Directory(_) = root {
        let candidates_for_deletion = find_subdirectories(root, file_arena, space_required);
        let smallest_deletion_candidate = candidates_for_deletion.iter().min_by_key(|d| d.get_size(file_arena)).unwrap();
        println!("Size of smallest directory to delete: {}", smallest_deletion_candidate.get_size(file_arena));
    }
}

fn main() {
    let file = fs::File::open("./src/input.txt").expect("Failed reading file");
    let reader = BufReader::new(file);

    let mut file_arena: FileArena = HashMap::new();
    file_arena.insert(String::from("/"), Node::Directory(DirectoryProperties{ name: String::from("/"), children: HashSet::new() }));
    let mut current_directory = PathBuf::from("/");
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if let Some(command) = Command::from_line(&line) {
            command.execute_command(&mut current_directory);
        } else {
            process_directory_list(&line, &current_directory, &mut file_arena);
        }
    }

    first_part(&file_arena);
    second_part(&file_arena);
}


mod test {
    use std::path::PathBuf;

    use crate::Command;

    #[test]
    fn is_command() {
        assert!(Command::is_command("$ cd"));
        assert!(Command::is_command("$ ls"));
        assert!(!Command::is_command("dir something"));
        assert!(!Command::is_command("45626 cvcbmcm"));
    }

    #[test]
    fn from_line() {
        assert_eq!(Command::from_line("$ ls"), Some(Command::List));
        assert_eq!(Command::from_line("$ cd my_path"), Some(Command::ChangeDirectory(String::from("my_path"))));
        assert_eq!(Command::from_line("$ cd some_dir"), Some(Command::ChangeDirectory(String::from("some_dir"))));
    }

    #[test]
    fn change_directoy_test() {
        let mut dir = PathBuf::from("/");
        Command::change_directory("..", &mut dir);
        assert_eq!(dir, PathBuf::from("/"));

        dir = PathBuf::from("/asdf");
        Command::change_directory("..", &mut dir);
        assert_eq!(dir, PathBuf::from("/"));

        dir = PathBuf::from("/asdf");
        Command::change_directory("lkjh", &mut dir);
        assert_eq!(dir, PathBuf::from("/asdf/lkjh"));
    }
}