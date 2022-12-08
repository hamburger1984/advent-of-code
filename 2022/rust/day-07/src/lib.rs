use std::{process::exit, collections::HashMap};

pub fn process_part1(input:&str) -> String {
    let mut dir_sizes = HashMap::new();
    let mut sub_dirs = Vec::new();
    let mut current_path = Vec::new();

    for line in input.lines() {
        let cmd_parts:Vec<&str> = line.split(' ').collect();

        match cmd_parts.as_slice() {
            ["$", "cd", ".."] => {
                current_path.pop();
                print!(" -> UP to {}\n", current_path.join("/"));
            }
            ["$", "cd", "/"] => {
                current_path.push("#");
                dir_sizes.insert(current_path.join("/"), 0);
                print!(" -> ROOT {}\n", current_path.join("/"));
            }
            ["$", "cd", dest] => {
                current_path.push(*dest);
                dir_sizes.insert(current_path.join("/"), 0);
                print!(" -> DOWN to {}\n", current_path.join("/"));
            }
            ["$", "ls"] => {
                print!(" -> LIST in {}\n", current_path.join("/"));
            }
            ["dir", child] => {
                print!(" -> SUBDIR {} in {}\n", child, current_path.join("/"));
                sub_dirs.push((current_path.join("/"), (*child).to_string()));
            }
            [file_size, file_name] => {
                print!(" -> FILE {}:{} in {}\n", file_name, file_size, current_path.join("/"));
                let parsed_file_size = file_size.parse::<i64>().unwrap();
                dir_sizes.entry(current_path.join("/")).and_modify(|s| { *s += parsed_file_size }).or_insert(0);
            }
            _ => {
                exit(1);
            }
        }
    }

    sub_dirs.sort_by(|a, b| { b.0.cmp(&a.0)});

    for (dir, subdir) in sub_dirs {
        let full_sub_path = [dir.clone(), subdir].join("/");
        let subdir_size = *dir_sizes.entry(full_sub_path).or_default();
        dir_sizes.entry(dir).and_modify(|s| { *s += subdir_size });
    }

    let mut sum = 0;
    for (_, size) in dir_sizes {
        if size < 100_000 {
            sum = sum + size;
        }
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "95437");
    }

    const TESTINPUT: &str =
    "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
}
