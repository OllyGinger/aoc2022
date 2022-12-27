fn process(data: &str) -> i32 {
    let mut cycles: Vec<i32> = Vec::new();
    let mut x: i32 = 1;
    for line in data.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        cycles.push(x);
        match parts[0] {
            "addx" => {
                let num = parts[1].parse::<i32>().unwrap();
                cycles.push(x);
                x += num;
            }
            &_ => {}
        };
    }

    // Simulate the operations
    let mut accum = 0;
    for i in [20i32, 60, 100, 140, 180, 220] {
        let val = cycles.get((i - 1) as usize).unwrap() * i;
        accum += val;
    }

    // Draw the CRT
    for cycle in 0..cycles.len() as i32 {
        if cycles[cycle as usize] <= (cycle % 40) + 1
            && (cycle % 40) + 1 <= cycles[cycle as usize] + 2
        {
            print!("#");
        } else {
            print!(".");
        }

        let rows = [40, 80, 120, 160, 200, 240];
        if rows.contains(&cycle) {
            println!("");
        }
    }

    accum
}

#[test]
fn test_example() {
    let example = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    assert_eq!(process(&example.to_string()), 13140);
}

pub fn run(data: &str) -> String {
    format!("Part 1: {}", process(data))
}
