use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

fn find_start_of_packet(packet: &str, start_of_packet: bool) -> usize {
    let mut history: VecDeque<char> = VecDeque::new();
    let mut idx = 0;
    let num_uniques_to_find = match start_of_packet {
        true => 4,
        false => 14,
    };

    for c in packet.chars() {
        // Check if history is all unique
        let history_set: HashSet<char> = HashSet::from_iter(history.iter().cloned());
        let mut all_unique = false;
        if history.len() == history_set.len() {
            all_unique = true;
        }

        if all_unique && history.len() == num_uniques_to_find {
            return idx;
        }

        if history.len() == num_uniques_to_find {
            history.pop_back();
        }
        history.push_front(c);
        idx += 1;
    }

    return 0;
}

#[test]
fn test_example() {
    assert_eq!(
        find_start_of_packet(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), true),
        7
    );
    assert_eq!(
        find_start_of_packet(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), true),
        5
    );
    assert_eq!(
        find_start_of_packet(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), true),
        6
    );
    assert_eq!(
        find_start_of_packet(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), true),
        10
    );
    assert_eq!(
        find_start_of_packet(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), true),
        11
    );

    assert_eq!(
        find_start_of_packet(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), false),
        19
    );
    assert_eq!(
        find_start_of_packet(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), false),
        23
    );
    assert_eq!(
        find_start_of_packet(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), false),
        23
    );
    assert_eq!(
        find_start_of_packet(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), false),
        29
    );
    assert_eq!(
        find_start_of_packet(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), false),
        26
    );
}

pub fn run(data: &String) -> String {
    return format!(
        "First packet marker: {}. First message marker: {}",
        find_start_of_packet(data, true),
        find_start_of_packet(data, false)
    );
}
