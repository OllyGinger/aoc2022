use std::collections::VecDeque;

enum Op {
    Add(u64),
    AddOld,
    Multiply(u64),
    MultiplyOld,
}

struct Monkey {
    item_worry: VecDeque<u64>,
    op: Op,
    test_divisible_by: u64,
    pass_to: (i32, i32),
    inspections: u64,
}

type MonkeyIndex = usize;

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            item_worry: VecDeque::new(),
            op: Op::Add(0),
            test_divisible_by: 0,
            pass_to: (0, 0),
            inspections: 0,
        }
    }

    fn add_worry(self: &mut Monkey, item: u64) {
        self.item_worry.push_back(item);
    }

    // Return (New monkey, new value)
    fn pick_new_monkey(
        self: &mut Monkey,
        item: u64,
        part1: bool,
        operand: u64,
    ) -> (MonkeyIndex, u64) {
        let mut new_value = item;
        match self.op {
            Op::Add(n) => new_value += n,
            Op::AddOld => new_value += new_value,
            Op::Multiply(n) => new_value *= n,
            Op::MultiplyOld => new_value *= new_value,
        }

        if part1 {
            // Get bored
            new_value /= 3;
        } else {
            new_value %= operand;
        }

        // Is divisible by
        let new_monkey;
        if new_value % self.test_divisible_by == 0 {
            new_monkey = self.pass_to.0;
        } else {
            new_monkey = self.pass_to.1;
        }

        self.inspections += 1;
        (new_monkey as usize, new_value)
    }
}

fn parse_monkeys(data: &str) -> Vec<Monkey> {
    let text = data.replace('\r', "");
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_spec in text.split("\n\n") {
        let mut on_true = 0i32;
        let mut on_false;
        let mut new_monkey = Monkey::new();
        for (i, line) in monkey_spec.lines().enumerate() {
            match i {
                1 => {
                    // Starting items
                    let parts = line.split_once(':').unwrap();
                    let items = parts
                        .1
                        .trim()
                        .split(',')
                        .map(|x| x.trim().parse::<u64>().unwrap())
                        .collect::<Vec<_>>();

                    new_monkey.item_worry.extend(items.iter());
                }
                2 => {
                    // Operation
                    let parts = line.split_once("old").unwrap();
                    let comps = parts
                        .1
                        .split_ascii_whitespace()
                        .map(|x| x.trim())
                        .collect::<Vec<_>>();
                    let op_chr = comps.get(0).unwrap().chars().nth(0).unwrap();
                    if let Ok(val) = comps.get(1).unwrap().parse::<u64>() {
                        match op_chr {
                            '*' => {
                                new_monkey.op = Op::Multiply(val);
                            }
                            '+' => {
                                new_monkey.op = Op::Add(val);
                            }
                            _ => {
                                panic!("Unknown op: {}", op_chr);
                            }
                        }
                    } else {
                        // Probably 'old * old'
                        if comps.get(1).unwrap().to_string() == "old" {
                            match op_chr {
                                '*' => {
                                    new_monkey.op = Op::MultiplyOld;
                                }
                                '+' => {
                                    new_monkey.op = Op::AddOld;
                                }
                                _ => {
                                    panic!("Unknown op: {}", op_chr);
                                }
                            }
                        }
                    }
                }
                3 => {
                    // Test
                    let mut parts = line.split_ascii_whitespace();
                    new_monkey.test_divisible_by = parts.nth(3).unwrap().parse::<u64>().unwrap();
                }
                4 => {
                    // On test true
                    let mut parts = line.split_ascii_whitespace();
                    on_true = parts.nth(5).unwrap().parse::<i32>().unwrap();
                }
                5 => {
                    // On test false
                    let mut parts = line.split_ascii_whitespace();
                    on_false = parts.nth(5).unwrap().parse::<i32>().unwrap();

                    new_monkey.pass_to = (on_true, on_false);
                }
                _ => {}
            }
        }

        monkeys.push(new_monkey);
    }

    monkeys
}

fn process(data: &str, part1: bool) -> u64 {
    let mut monkeys = parse_monkeys(data);
    let mut rounds = 20;
    if !part1 {
        rounds = 10_000;
    }

    let divisor: u64 = monkeys.iter().map(|x| x.test_divisible_by).product();

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].item_worry.pop_front() {
                let (new_monkey, new_value) = monkeys[m].pick_new_monkey(item, part1, divisor);
                monkeys[new_monkey].add_worry(new_value);
            }
        }
    }

    let mut res = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();
    res.sort();

    res[res.len() - 1] * res[res.len() - 2]
}

#[test]
fn test_example() {
    let example = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    assert_eq!(process(example, true), 10605);
    assert_eq!(process(example, false), 2_713_310_158);
}

pub fn run(data: &str) -> String {
    format!(
        "Part 1: {} - Part 2: {}",
        process(data, true),
        process(data, false)
    )
}
