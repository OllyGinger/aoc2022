use std::collections::HashSet;

type RucksackPriority = u32;

fn split_rucksack_items(items: &String) -> (&str, &str) {
    items.split_at(items.len() / 2)
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

fn get_common_item(compartment1: &str, compartment2: &str) -> char {
    let set1: HashSet<char> = compartment1.chars().collect();
    let set2: HashSet<char> = compartment2.chars().collect();

    let common_items: HashSet<char> = set1.intersection(&set2).cloned().collect();
    assert_eq!(common_items.len(), 1);
    return common_items.iter().next().cloned().unwrap();
}

fn get_rucksack_priority(items: &String) -> RucksackPriority {
    let compartments = split_rucksack_items(items);
    let common = get_common_item(compartments.1, compartments.1);
    get_priority(common)
}

#[test]
fn test_example() {
    let exampleInput = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    // Check splitting works
    assert_eq!(
        split_rucksack_items(&"vJrwpWtwJgWrhcsFMMfFFhFp".to_owned()),
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
    );

    // Check priority
    assert_eq!(get_priority('a'), 1);
    assert_eq!(get_priority('b'), 2);
    assert_eq!(get_priority('z'), 26);
    assert_eq!(get_priority('A'), 27);
    assert_eq!(get_priority('Z'), 52);

    // Check common item
    assert_eq!(
        get_common_item(&"vJrwpWtwJgWr".to_owned(), &"hcsFMMfFFhFp".to_owned()),
        'p'
    );

    // Check rucksack priority
    assert_eq!(
        get_rucksack_priority(&"vJrwpWtwJgWrhcsFMMfFFhFp".to_string()),
        16
    );

    for line in exampleInput.lines() {
        let compartments = split_rucksack_items(&line.to_string());
    }
}

pub fn run(data: &String) -> String {
    "".to_string()
}
