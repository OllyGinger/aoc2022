enum GameMove {
    Paper,
    Rock,
    Scissors,
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

fn winsAgainst(a: &GameMove, b: &GameMove) -> GameResult {
    match a {
        GameMove::Paper => match b {
            GameMove::Paper => GameResult::Draw,
            GameMove::Rock => GameResult::Win,
            GameMove::Scissors => GameResult::Loss,
        },
        GameMove::Rock => match b {
            GameMove::Paper => GameResult::Loss,
            GameMove::Rock => GameResult::Draw,
            GameMove::Scissors => GameResult::Win,
        },
        GameMove::Scissors => match b {
            GameMove::Paper => GameResult::Win,
            GameMove::Rock => GameResult::Loss,
            GameMove::Scissors => GameResult::Draw,
        },
    }
}

fn getMove(c: char) -> GameMove {
    match c {
        'A' | 'X' => GameMove::Rock,
        'B' | 'Y' => GameMove::Paper,
        'C' | 'Z' => GameMove::Scissors,
        _ => GameMove::Rock,
    }
}

fn getPoints(my_move: &GameMove, their_move: &GameMove) -> i32 {
    let result = winsAgainst(my_move, their_move);
    let mut points = 0;
    points = match my_move {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissors => 3,
    };

    match result {
        GameResult::Win => points += 6,
        GameResult::Loss => {}
        GameResult::Draw => points += 3,
    }

    points
}

fn playGame(data: &String) -> i32 {
    let mut my_points = 0;
    for line in data.lines() {
        let parts = line.split_once(' ').unwrap();

        let move1 = getMove(parts.0.chars().nth(0).unwrap());
        let move2 = getMove(parts.1.chars().nth(0).unwrap());

        let points = getPoints(&move2, &move1);
        my_points += points;
    }
    my_points
}

#[test]
fn test_example() {
    let input = r#"A Y
B X
C Z"#;

    let pts = playGame(&input.to_string());
    assert_eq!(pts, 15);
}

pub fn run(data: &String) -> String {
    let mut my_points = playGame(data);
    return format!("My Points: {}", my_points);
}
