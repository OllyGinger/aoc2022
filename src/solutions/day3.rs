use std::collections::HashSet;

type RucksackPriority = u32;

fn split_rucksack_items(items: &String) -> (&str, &str) {
    items.split_at(items.len() / 2)
}

#[test]
fn test_splitting() {
    // Check splitting works
    assert_eq!(
        split_rucksack_items(&"vJrwpWtwJgWrhcsFMMfFFhFp".to_owned()),
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
    );
}

fn get_priority(c: char) -> RucksackPriority {
    if c.is_lowercase() {
        return (c as RucksackPriority - 'a' as RucksackPriority) + 1;
    } else if c.is_uppercase() {
        return (c as RucksackPriority - 'A' as RucksackPriority) + 27;
    } else {
        // Return 0 for any other character that is not a lowercase or uppercase letter
        return 0;
    }
}

#[test]
fn test_priority() {
    // Check priority
    assert_eq!(get_priority('a'), 1);
    assert_eq!(get_priority('b'), 2);
    assert_eq!(get_priority('z'), 26);
    assert_eq!(get_priority('A'), 27);
    assert_eq!(get_priority('Z'), 52);
}

fn get_common_item(compartment1: &str, compartment2: &str) -> char {
    let set1: HashSet<char> = compartment1.chars().collect();
    let set2: HashSet<char> = compartment2.chars().collect();

    let common_items: HashSet<char> = set1.intersection(&set2).cloned().collect();
    assert_eq!(common_items.len(), 1);
    return common_items.iter().next().cloned().unwrap();
}

fn get_common_item_generic<'a, I>(item_strings: I) -> char
where
    I: Iterator<Item = &'a str>,
{
    let mut accumulation: HashSet<char> = HashSet::new();
    for items in item_strings {
        let set: HashSet<char> = items.chars().collect();
        let common_items: HashSet<char> = accumulation.intersection(&set).cloned().collect();
        if accumulation.len() == 0 {
            accumulation = set;
        } else {
            accumulation = common_items;
        }
    }

    assert_eq!(accumulation.len(), 1);
    accumulation.iter().next().cloned().unwrap()
}

#[test]
fn test_common_item() {
    // Check common item
    assert_eq!(
        get_common_item(&"vJrwpWtwJgWr".to_owned(), &"hcsFMMfFFhFp".to_owned()),
        'p'
    );

    // Check generic common item
    let mut str_list: HashSet<&str> = HashSet::new();
    str_list.insert(&"vJrwpWtwJgWr".clone());
    str_list.insert(&"hcsFMMfFFhFp".clone());

    assert_eq!(get_common_item_generic(str_list.iter().copied()), 'p');
}

fn get_rucksack_priority(items: &String) -> RucksackPriority {
    let compartments = split_rucksack_items(items);
    let common = get_common_item(compartments.0, compartments.1);
    get_priority(common)
}

#[test]
fn test_rucksack_priority() {
    // Check rucksack priority
    assert_eq!(
        get_rucksack_priority(&"vJrwpWtwJgWrhcsFMMfFFhFp".to_string()),
        16
    );
}

#[test]
fn test_example() {
    let example_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    let mut priorities: RucksackPriority = 0;
    for line in example_input.lines() {
        let priority = get_rucksack_priority(&line.to_string());
        priorities += priority;
    }
    assert_eq!(priorities, 157);

    let mut priorities2 = 0;
    for i in example_input.lines().collect::<Vec<_>>().chunks(3) {
        let mut bags: HashSet<&str> = HashSet::new();
        for line in i {
            bags.insert(line);
        }

        let common = get_common_item_generic(bags.iter().copied());
        priorities2 += get_priority(common)
    }
    assert_eq!(priorities2, 70);
}

pub fn run(data: &String) -> String {
    let mut priorities: RucksackPriority = 0;
    for line in data.lines() {
        let priority = get_rucksack_priority(&line.to_string());
        priorities += priority;
    }

    // Part 2
    let mut priorities2 = 0;
    for i in data.lines().collect::<Vec<_>>().chunks(3) {
        let mut bags: HashSet<&str> = HashSet::new();
        for line in i {
            bags.insert(line);
        }

        let common = get_common_item_generic(bags.iter().copied());
        priorities2 += get_priority(common)
    }

    return format!(
        "Rucksack priorities: {}\nPart 2 priorities: {}",
        priorities, priorities2
    );
}
