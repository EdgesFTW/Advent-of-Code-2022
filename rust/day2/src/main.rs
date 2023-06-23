use std::env;

pub mod day2 {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    enum Action {
        Rock,
        Paper,
        Scissors,
    }

    enum GameResult {
        Win,
        Tie,
        Lose,
    }

    fn calculate_value_pt1(opponent_guess: Action, player_guess: Action) -> i32 {
        let mut score = 0;

        // basic case
        match player_guess {
            Action::Rock => score += 1,
            Action::Paper => score += 2,
            Action::Scissors => score += 3,
        }

        // check for tie
        match (&opponent_guess, &player_guess) {
            (Action::Rock, Action::Rock) => score += 3,
            (Action::Paper, Action::Paper) => score += 3,
            (Action::Scissors, Action::Scissors) => score += 3,
            _ => {}
        }

        // check for player winning
        match (&opponent_guess, &player_guess) {
            (Action::Rock, Action::Paper) => score += 6,
            (Action::Paper, Action::Scissors) => score += 6,
            (Action::Scissors, Action::Rock) => score += 6,
            _ => {}
        }

        score
    }

    pub fn day2_pt1(filename: String) -> std::io::Result<i32> {
        let file = File::open(filename)?;
        let mut result = 0;
        let file = BufReader::new(file);
        for line in file.lines() {
            let string = line.unwrap();
            let opponent_guess: Action = match &string[0..1] {
                str if str == "A" => Action::Rock,
                str if str == "B" => Action::Paper,
                str if str == "C" => Action::Scissors,
                _ => panic!("invalid opponent_guess"),
            };
            let player_guess: Action = match &string[2..3] {
                str if str == "X" => Action::Rock,
                str if str == "Y" => Action::Paper,
                str if str == "Z" => Action::Scissors,
                _ => panic!("invalid opponent_guess"),
            };
            result += calculate_value_pt1(opponent_guess, player_guess);
        }

        Ok(result)
    }

    fn calculate_value_pt2(player_guess: Action, player_game_result: GameResult) -> i32 {
        let mut score = 0;

        // basic case
        match player_guess {
            Action::Rock => score += 1,
            Action::Paper => score += 2,
            Action::Scissors => score += 3,
        }

        // check for game conditions
        match player_game_result {
            GameResult::Win => score += 6,
            GameResult::Tie => score += 3,
            GameResult::Lose => score += 0,
        }

        score
    }

    pub fn day2_pt2(filename: String) -> std::io::Result<i32> {
        let file = File::open(filename)?;
        let mut result = 0;
        let file = BufReader::new(file);
        for line in file.lines() {
            let string = line.unwrap();
            let opponent_guess: Action = match &string[0..1] {
                str if str == "A" => Action::Rock,
                str if str == "B" => Action::Paper,
                str if str == "C" => Action::Scissors,
                _ => panic!("invalid opponent_guess"),
            };
            let player_game_result: GameResult = match &string[2..3] {
                str if str == "X" => GameResult::Lose,
                str if str == "Y" => GameResult::Tie,
                str if str == "Z" => GameResult::Win,
                _ => panic!("invalid opponent_guess"),
            };
            let player_guess: Action = match opponent_guess {
                Action::Rock => match player_game_result {
                    GameResult::Win => Action::Paper,
                    GameResult::Lose => Action::Scissors,
                    GameResult::Tie => Action::Rock,
                },
                Action::Paper => match player_game_result {
                    GameResult::Win => Action::Scissors,
                    GameResult::Lose => Action::Rock,
                    GameResult::Tie => Action::Paper,
                },
                Action::Scissors => match player_game_result {
                    GameResult::Win => Action::Rock,
                    GameResult::Lose => Action::Paper,
                    GameResult::Tie => Action::Scissors,
                },
            };
            result += calculate_value_pt2(player_guess, player_game_result);
        }

        Ok(result)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let result: i32 = match &args[1] {
        x if x == "part1" => day2::day2_pt1(String::from("input.txt")).unwrap(),
        x if x == "part2" => day2::day2_pt2(String::from("input.txt")).unwrap(),
        _ => panic!("use arguments 'part1' or 'part2'."),
    };

    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day2::day2_pt1;
    use crate::day2::day2_pt2;

    #[test]
    fn given_pt1() {
        let result = day2_pt1(String::from("test.txt")).unwrap();

        println!("Result: {}", result);
        assert!(result == 15);
    }

    #[test]
    fn given_pt2() {
        let result = day2_pt2(String::from("test.txt")).unwrap();

        println!("Result: {}", result);
        assert!(result == 12);
    }
}
