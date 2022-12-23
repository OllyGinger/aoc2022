#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Node>,
    is_dir: bool,
}

impl Node {
    fn new(name: String) -> Node {
        Self {
            name,
            size: 0,
            children: Vec::new(),
            is_dir: false,
        }
    }

    fn find_path(&mut self, path_stack: &Vec<&str>) -> &mut Self {
        let mut current = self;
        for path in path_stack {
            current = current
                .children
                .iter_mut()
                .find(|f| f.name == *path)
                .unwrap();
        }
        current
    }

    fn update_sizes(&mut self) -> usize {
        for c in &mut self.children {
            self.size += c.update_sizes();
        }

        self.size
    }

    fn all_children(&self) -> Vec<&Node> {
        let mut ret = Vec::new();
        for c in &self.children {
            ret.push(c.clone());
            ret.extend(c.all_children())
        }
        ret
    }
}

fn generate_filesystem(commands: &str) -> Node {
    let mut root_node: Node = Node::new("".to_string().to_owned());
    root_node.is_dir = true;
    let mut path_stack = Vec::new();

    for line in commands.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[..2] == ["$", "cd"] {
            // Change directory
            if parts[2] == "/" {
                path_stack.clear();
                continue;
            // Up a directory
            } else if parts[2] == ".." {
                path_stack.pop();
                continue;
            // Go to specific dir
            } else {
                let parent = root_node.find_path(&path_stack);
                path_stack.push(parts[2]);
                if parent.children.iter().any(|x| x.name == parts[2]) {
                    continue;
                }

                parent.children.push(Node::new(parts[2].to_string()))
            }
        } else if parts[0] == "dir" {
            let parent = root_node.find_path(&path_stack);
            if let Some(d) = parent.children.iter_mut().find(|x| x.name == parts[1]) {
                d.is_dir = true;
                continue;
            }

            let mut child = Node::new(parts[1].to_string());
            child.is_dir = true;
            parent.children.push(child);
        }

        if let Ok(size) = parts[0].parse::<usize>() {
            let mut file = Node::new(parts[1].to_string());
            file.size = size;
            root_node.find_path(&path_stack).children.push(file)
        }
    }

    root_node.update_sizes();
    root_node
}

#[test]
fn test_example() {
    let example = r#"$ cd /
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
7214296 k"#;
    assert_eq!(
        generate_filesystem(example)
            .all_children()
            .iter()
            .filter(|x| x.is_dir && x.size < 100000)
            .fold(0, |a, x| a + x.size)
            .to_string(),
        "95437"
    );
}

pub fn run(data: &String) -> String {
    let fs = generate_filesystem(data);
    let part1 = fs
        .all_children()
        .iter()
        .filter(|x| x.is_dir && x.size < 100000)
        .fold(0, |a, x| a + x.size)
        .to_string();

    let needed_space = 30000000 - (70000000 - fs.size);

    let folder_vec = fs.all_children();
    let mut folder_vec = folder_vec.iter().collect::<Vec<_>>();
    folder_vec.sort_by(|a, b| a.size.cmp(&b.size));

    let part2 = folder_vec
        .iter()
        .find(|x| x.is_dir && x.size > needed_space)
        .unwrap()
        .size
        .to_string();

    return format!("Part1: {}, Part2: {}", part1, part2);
}
