use std::io::{self, BufRead};

fn team_formation(scores: &mut Vec<u64>, team_size: usize, k: usize) {
    let mut team_scores: Vec<u64> = Vec::with_capacity(team_size);

    while team_scores.len() < team_size {
        let selected;

        if k < scores.len() {
            let front = scores
                .get(0..k)
                .expect("error getting first k elements of scores array");
            let back = scores
                .get(scores.len() - k..)
                .expect("error getting last k elements of scores array");
            let front_max = *front
                .iter()
                .max()
                .expect("error finding maximum score in front array");
            let back_max = *front
                .iter()
                .max()
                .expect("error finding maximum score in back array");

            if front_max >= back_max {
                selected = front_max;
                let index = front
                    .iter()
                    .position(|&e| e == front_max)
                    .expect("error finding maximum index in front scores array");
                scores.swap_remove(index);
            } else {
                selected = back_max;
                let index = back
                    .iter()
                    .position(|&e| e == back_max)
                    .expect("error finding maximum index in back scores array");
                scores.swap_remove(index);
            }
        } else {
            selected = *scores.iter().max().expect("error finding maximum score");
            let index = scores
                .iter()
                .position(|&e| e == selected)
                .expect("error finding maximum index in scores array");
            scores.swap_remove(index);
        }

        team_scores.push(selected);
    }

    println!("{}", team_scores.iter().sum::<u64>());
}

fn main() -> Result<(), std::num::ParseIntError> {
    let stdin = io::stdin();
    let mut line = String::new();

    // Read the number of scores
    stdin.lock().read_line(&mut line).unwrap();
    let score_count = line.trim().parse::<usize>()?;
    line.clear();

    // Create a container to hold the scores
    let mut scores: Vec<u64> = Vec::with_capacity(score_count);

    // Read scores
    for _ in 0..score_count {
        // Read the score
        stdin.lock().read_line(&mut line).unwrap();
        let score = line.trim().parse::<u64>()?;
        line.clear();
        scores.push(score);
    }

    // Read the team size
    stdin.lock().read_line(&mut line).unwrap();
    let team_size = line.trim().parse::<usize>()?;
    line.clear();

    // Read the value k
    stdin.lock().read_line(&mut line).unwrap();
    let k = line.trim().parse::<usize>()?;
    line.clear();

    team_formation(&mut scores, team_size, k);

    Ok(())
}
