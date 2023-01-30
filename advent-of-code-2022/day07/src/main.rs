use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let cmds = parse_cmds(&contents);
    let dirs = build_filesystem(cmds);

    print_tree(&dirs, "/", 0);

    let result: u32 = dirs
        .keys()
        .map(|name| {
            let item = &**dirs.get(name).unwrap();
            let size = dir_size(item);
            println!("{name} has size of {size}.");
            size
        })
        .filter(|x| x < &100_000)
        .sum();

    println!("Sum of directories smaller than 100.000 is {result}.");
}

fn parse_cmds(contents: &str) -> Vec<Cmd> {
    let cmds: Vec<Cmd> = contents
        .split("$")
        .skip(1)
        .map(|s| {
            let new_s = s.replace("\r\n", " ");
            let (cmd_str, arg_str) = new_s.trim().split_once(" ").unwrap();
            match (cmd_str, arg_str) {
                ("cd", _) => Cmd::Cd(arg_str.to_string()),
                ("ls", _) => {
                    let words: Vec<&str> = arg_str.split(" ").collect();
                    let items: Vec<Item> = words
                        .chunks(2)
                        .map(|chunk| match chunk[0].parse::<u32>() {
                            Ok(size) => Item::File(chunk[1].to_string(), size),
                            Err(_) => Item::Dir(chunk[1].to_string(), vec![], String::from("")),
                        })
                        .collect();

                    Cmd::Ls(items)
                }
                _ => Cmd::None,
            }
        })
        .collect();

    cmds
}

fn build_filesystem(cmds: Vec<Cmd>) -> HashMap<String, Box<Item>> {
    let mut dirs: HashMap<String, Box<Item>> = HashMap::new();
    let mut current = String::from("/");

    for cmd in cmds {
        match cmd {
            Cmd::Cd(name) => {
                if name == ".." {
                    let boxed_current = dirs.get_mut(&current).unwrap();
                    if let Item::Dir(_, _, parent) = &mut **boxed_current {
                        current = parent.to_string();
                    }
                } else {
                    let dir: Item = Item::Dir(name.to_string(), vec![], current);
                    current = String::from(name.to_string());

                    match dirs.entry(name) {
                        Entry::Occupied(_) => (),
                        Entry::Vacant(e) => {
                            e.insert(Box::new(dir));
                        }
                    }
                }
            }
            Cmd::Ls(items) => {
                for item in items {
                    let boxed_current = dirs.get_mut(&current).unwrap();

                    match item {
                        Item::Dir(name, _, _) => {
                            if let Item::Dir(parent_name, parent_items, _) = &mut **boxed_current {
                                let new_dir = Item::Dir(name, vec![], parent_name.to_string());
                                parent_items.push(Box::new(new_dir));
                            }
                        }
                        Item::File(name, size) => {
                            if let Item::Dir(_, parent_items, _) = &mut **boxed_current {
                                let new_file = Item::File(name, size);
                                parent_items.push(Box::new(new_file));
                            }
                        }
                    }
                }
            }
            Cmd::None => (),
        };
    }

    dirs
}

fn print_tree(dirs: &HashMap<String, Box<Item>>, root: &str, indent: usize) {
    let root: &Item = &**(dirs.get(root).unwrap());

    if let Item::Dir(name, items, _) = root {
        println!("{:indent$}{name} (dir)", "", indent = indent);

        for item in items {
            match &**item {
                Item::File(name, size) => println!(
                    "{:indent$}{name} (file, size={size})",
                    "",
                    indent = indent + 4
                ),
                Item::Dir(name, ..) => {
                    print_tree(&dirs, &name, indent + 4);
                }
            };
        }
    }
}

fn dir_size(item: &Item) -> u32 {
    match item {
        Item::File(_, size) => size.to_owned(),
        Item::Dir(_, items, ..) => {
            let total_size: u32 = items
                .iter()
                .map(|x| match **x {
                    Item::File(_, size) => size,
                    Item::Dir(..) => dir_size(&**x),
                })
                .sum();

            total_size
        }
    }
}

#[derive(Debug)]
enum Item {
    File(String, u32),
    Dir(String, Vec<Box<Item>>, String),
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls(Vec<Item>),
    None,
}
