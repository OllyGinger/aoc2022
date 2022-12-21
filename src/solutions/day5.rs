fn process(data: &String, part2: bool) -> String {
    let data = data.replace("\r", "");
    let (crates, orders) = data.split_once("\n\n").unwrap();
    let mut crates = process_crates(crates);
    for line in orders.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let num_to_move = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        let count = crates[from].len() - num_to_move..;
        let mut working = crates[from].drain(count).collect::<Vec<_>>();
        if !part2 {
            working.reverse();
        }
        crates[to].extend(working);
    }

    crates
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|mut x| x.pop().unwrap())
        .collect()
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
    assert_eq!(process(&example.to_string(), false), "CMZ".to_owned());
}

pub fn run(data: &String) -> String {
    return format!(
        "Top of each stack: '{}'\nPart 2: '{}'",
        process(&data.to_string(), false),
        process(&data.to_string(), true)
    );
}
