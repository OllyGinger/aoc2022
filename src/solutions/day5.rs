fn process(data: &String) -> String {
    let (crates, orders) = data.split_once("\n\n").unwrap();
    let mut crates = process_crates(crates);

    return String::new();
}

fn process_crates(crates: &str) -> Vec<Vec<char>> {
    let len = crates.lines().next().unwrap().len() + 1;
    let mut out = vec![Vec::new(); len / 4];

    for i in crates.lines().filter(|x| !x.starts_with(" 1")) {
        for (ji, i) in i.chars().enumerate().filter(|x| x.1.is_ascii_uppercase()) {
            out[(ji as f32 / 4.).ceil() as usize - 1].insert(0, i);
        }
    }

    out
}

#[test]
fn test_example() {
    let example = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    process(&example.to_string());
}

pub fn run(data: &String) -> String {
    return format!("Rucksack priorities: \nPart 2 priorities:");
}
