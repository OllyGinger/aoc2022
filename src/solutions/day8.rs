fn get_trees(input: &String) -> Vec<Vec<usize>> {
    let lines = input.lines();
    lines
        .map(|x| {
            x.chars()
                .filter(|n| n.is_ascii_digit())
                .filter_map(|n| n.to_digit(10))
                .map(|n| n as usize)
                .collect()
        })
        .collect()
}

fn get_visible_trees(input: &String) -> usize {
    let trees = get_trees(&input);
    let mut count = 0_usize;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let height = trees[row][col];
            if trees[..row].iter().all(|x| x[col] < height)
                || trees[row][..col].iter().all(|x| x < &height)
                || trees[row + 1..].iter().all(|x| x[col] < height)
                || trees[row][col + 1..].iter().all(|x| x < &height)
            {
                count += 1;
            }
        }
    }

    count
}

fn get_scenic_score(trees: &Vec<Vec<usize>>, coord: (usize, usize)) -> usize {
    let tree_height = trees[coord.0][coord.1];

    let looking_right = process_line(
        tree_height,
        trees[coord.0][coord.1 + 1..].iter().map(|t| *t),
    );
    let looking_left = process_line(
        tree_height,
        trees[coord.0][..coord.1].iter().rev().map(|t| *t),
    );
    let looking_down = process_line(tree_height, trees[coord.0 + 1..].iter().map(|t| t[coord.1]));
    let looking_up = process_line(
        tree_height,
        trees[..coord.0].iter().rev().map(|t| t[coord.1]),
    );

    looking_right * looking_left * looking_down * looking_up
}

fn process_line(current_tree: usize, it: impl Iterator<Item = usize>) -> usize {
    let mut score = 0;
    for t in it {
        score += 1;

        if t >= current_tree {
            break;
        }
    }
    score
}

fn get_highest_scenic_score(input: &String) -> usize {
    let trees = get_trees(input);
    let mut highest_score = 0;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let score = get_scenic_score(&trees, (row, col));
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

#[test]
fn test_example() {
    let example = r#"30373
25512
65332
33549
35390"#;

    assert_eq!(get_visible_trees(&example.to_string()), 21);
    assert_eq!(get_highest_scenic_score(&example.to_string()), 8);
}

pub fn run(data: &String) -> String {
    return format!(
        "Part1: {} - Part2: {}",
        get_visible_trees(data),
        get_highest_scenic_score(data)
    );
}
