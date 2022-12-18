use std::collections::HashSet;

type Assignment = u32;
type AssignmentRange = (Assignment, Assignment);
type ElfPairAssignment = (AssignmentRange, AssignmentRange);

// Parse the spec: `00-00,00-00`
fn parse_spec(spec: &String) -> ElfPairAssignment {
    match spec.find(",") {
        None => panic!("Invalid spec, missing `,`: {}", spec),
        Some(part_idx) => {
            // This will leave us with something like
            let left = &spec[..part_idx];
            let right = &spec[part_idx + 1..];
            let l = parse_range(&left.to_string());
            let r = parse_range(&right.to_string());
            (l, r)
        }
    }
}

// Parse the `00-00` range
fn parse_range(s: &String) -> AssignmentRange {
    match s.find("-") {
        None => panic!("Invalid spec, missing `,`: {}", &s),
        Some(part_idx) => {
            match (
                Assignment::from_str_radix(&s[..part_idx], 10),
                Assignment::from_str_radix(&s[part_idx + 1..], 10),
            ) {
                (Ok(l), Ok(r)) => (l, r),
                _ => panic!("Invalid spec, no Assignment's found: {}", &s),
            }
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(parse_spec(&"2-4,6-8".to_owned()), ((2, 4), (6, 8)));
    assert_eq!(parse_spec(&"20-40,06-80".to_owned()), ((20, 40), (6, 80)));
}

fn any_range_fully_overlaps_other(pair: ElfPairAssignment) -> bool {
    (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1)
        || (pair.1 .0 <= pair.0 .0 && pair.1 .1 >= pair.0 .1)
}

#[test]
fn test_any_range_fully_overlaps_other() {
    assert_eq!(any_range_fully_overlaps_other(((0, 1), (2, 4))), false);
    assert_eq!(any_range_fully_overlaps_other(((10, 100), (40, 99))), true);
    assert_eq!(any_range_fully_overlaps_other(((10, 20), (4, 50))), true);
}

fn any_range_partially_overlaps_other(pair: ElfPairAssignment) -> bool {
    // .23......  2-3
    // ...45....  4-5

    // ......789  7-9
    // ....567..  5-7
    (pair.0 .0 == pair.1 .0 || pair.0 .1 == pair.1 .1)
        || (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .0)
        || (pair.1 .0 <= pair.0 .0 && pair.1 .1 >= pair.0 .0)
}

#[test]
fn test_any_range_partially_overlaps_other() {
    assert_eq!(any_range_partially_overlaps_other(((0, 5), (3, 10))), true);
    assert_eq!(
        any_range_partially_overlaps_other(((10, 100), (100, 199))),
        true
    );
    assert_eq!(
        any_range_partially_overlaps_other(((3, 100), (101, 200))),
        false
    );
}

#[test]
fn test_example() {
    let example = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    let mut broken_assignments = 0;
    let mut partially_broken_assignments = 0;
    for line in example.lines() {
        let pair = parse_spec(&line.to_string());
        if any_range_fully_overlaps_other(pair) {
            broken_assignments += 1;
        }

        if any_range_partially_overlaps_other(pair) {
            partially_broken_assignments += 1;
        }
    }

    assert_eq!(broken_assignments, 2);
    assert_eq!(partially_broken_assignments, 4);
}

pub fn run(data: &String) -> String {
    let mut broken_assignments = 0;
    let mut partially_broken_assignments = 0;
    for line in data.lines() {
        let pair = parse_spec(&line.to_string());
        if any_range_fully_overlaps_other(pair) {
            broken_assignments += 1;
        }
        if any_range_partially_overlaps_other(pair) {
            partially_broken_assignments += 1;
        }
    }

    return format!(
        "Broken assignments: {}\nFully broken assignments: {}",
        broken_assignments, partially_broken_assignments
    );
}
