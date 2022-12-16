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

fn wins_against(my_move: &GameMove, their_move: &GameMove) -> GameResult {
    match my_move {
        GameMove::Paper => match their_move {
            GameMove::Paper => GameResult::Draw,
            GameMove::Rock => GameResult::Win,
            GameMove::Scissors => GameResult::Loss,
        },
        GameMove::Rock => match their_move {
            GameMove::Paper => GameResult::Loss,
            GameMove::Rock => GameResult::Draw,
            GameMove::Scissors => GameResult::Win,
        },
        GameMove::Scissors => match their_move {
            GameMove::Paper => GameResult::Win,
            GameMove::Rock => GameResult::Loss,
            GameMove::Scissors => GameResult::Draw,
        },
    }
}

fn get_move_for_result(their_move: &GameMove, target_result: &GameResult) -> GameMove {
    match their_move {
        GameMove::Paper => match target_result {
            GameResult::Win => GameMove::Scissors,
            GameResult::Loss => GameMove::Rock,
            GameResult::Draw => GameMove::Paper,
        },
        GameMove::Rock => match target_result {
            GameResult::Win => GameMove::Paper,
            GameResult::Loss => GameMove::Scissors,
            GameResult::Draw => GameMove::Rock,
        },
        GameMove::Scissors => match target_result {
            GameResult::Win => GameMove::Rock,
            GameResult::Loss => GameMove::Paper,
            GameResult::Draw => GameMove::Scissors,
        },
    }
}

fn get_move(c: char) -> GameMove {
    match c {
        'A' | 'X' => GameMove::Rock,
        'B' | 'Y' => GameMove::Paper,
        'C' | 'Z' => GameMove::Scissors,
        _ => GameMove::Rock,
    }
}

fn get_move_part_two(their_move: &GameMove, c: char) -> GameMove {
    let target_move = get_target_game_result(c);
    get_move_for_result(their_move, &target_move)
}

fn get_target_game_result(c: char) -> GameResult {
    match c {
        'X' => GameResult::Loss,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => GameResult::Draw,
    }
}

fn get_points(my_move: &GameMove, their_move: &GameMove) -> i32 {
    let result = wins_against(my_move, their_move);
    let mut points;
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

fn play_game(data: &String, part_two_strat: bool) -> i32 {
    let mut my_points = 0;
    for line in data.lines() {
        let parts = line.split_once(' ').unwrap();

        let their_move = get_move(parts.0.chars().nth(0).unwrap());
        let my_move: GameMove;
        if !part_two_strat {
            my_move = get_move(parts.1.chars().nth(0).unwrap());
        } else {
            my_move = get_move_part_two(&their_move, parts.1.chars().nth(0).unwrap())
        }

        let points = get_points(&my_move, &their_move);
        my_points += points;
    }
    my_points
}

#[test]
fn test_example() {
    let input = r#"A Y
B X
C Z"#;

    let pts = play_game(&input.to_string(), false);
    assert_eq!(pts, 15);

    let part_two_pts = play_game(&input.to_string(), true);
    assert_eq!(part_two_pts, 12);
}

pub fn run(data: &String) -> String {
    let my_points = play_game(data, false);
    let part_two_points = play_game(data, true);
    return format!(
        "My Points: {} - Part two points: {}",
        my_points, part_two_points
    );
}
