/*
Day 21: Dirac Dice

2 players play on a board of 10 spaces, counting from 1 to 10, then back to 1.
Each player rolls 3 dice, adds them up, and moves the number of spaces.
The value of the space they are on is added to that player's score.
The starting positions of players 1 and 2 are the puzzle input.

Part 1: use a deterministic dice that always rolls in order 1,2,3,...,100
The game ends as soon as 1 player scores 1000 or more.
Multiply the losing players score by the number of times the dice was rolled.

Part 2: This time play with a dirac dice with 3 sides.
Each role creates separate universes where each possible value is rolled.
A universe is finished when one player scores 21 or more.
Find the player who wins the most universes, how many universes does that player win?
*/

use std::cmp;
use std::collections::HashMap;

// Part 1 deterministic die struct
#[derive(Debug)]
struct DeterministicDie {
    roll: i32,
    num_roles: i32,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie {
            roll: 0,
            num_roles: 0,
        }
    }
    
    fn roll(&mut self) -> i32 {
        self.roll += 1;
        self.num_roles += 1;
        if self.roll > 100 {
            self.roll = 1;
        }
        return self.roll;
    }
}

// Part 2 universe tracker
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Universe {
    p1_score: i32,
    p2_score: i32,
    p1_position: i32,
    p2_position: i32
}

impl Universe {
    fn move_p1(&mut self, new_position: i32) {
        self.p1_position = new_position;
        self.p1_score += new_position;
    }

    fn move_p2(&mut self, new_position: i32) {
        self.p2_position = new_position;
        self.p2_score += new_position;
    }

    fn p1_win(&self) -> Option<bool> {
        if self.p2_score < 21 && self.p1_score < 21 {
            return None;
        }
        return Some(self.p1_score > self.p2_score);
    }
}

//Part 1: Play the game out one roll at a time with the deterministic dice
pub fn play_deterministic(p1_start: i32, p2_start: i32) -> i32 { 
    // each entry is a player with (total_score, current_position)
    let mut players: Vec<(i32, i32)> = Vec::new();
    players.push((0, p1_start));
    players.push((0, p2_start));
    let mut die = DeterministicDie::new();

    // game ends when the first player reaches 1000
    while players.iter().map(|&(score, _)| score).max().unwrap() < 1000 {
        for i in 0..players.len() {
            let (score, position) = players[i];
            let roll = die.roll() + die.roll() + die.roll();
            let next_pos = calc_position(position, roll);
            let next_score = score + next_pos;
            players[i] = (next_score, next_pos);
            if next_score >= 1000 {
                // player reached 1000, stop the loop before the next player rolls
                break;
            }
        }
    }
    return players.into_iter().map(|(score, _)| score).min().unwrap() * die.num_roles;
}

// Part 2: recursive DFS with memoization
// each player can have a score of 0 - 20 and position 1-10
// This gives a worst case of 44100 states to track (reality is 14222)
// runs in ~2 seconds
pub fn dirac_dice(p1_start: i32, p2_start: i32) -> usize {
    let initial_universe = Universe {
        p1_score: 0,
        p2_score: 0,
        p1_position: p1_start,
        p2_position: p2_start
    };

    // memoize the universe state and how many player 1 and player 2 wins happen for that state
    let mut memo: HashMap<Universe, (usize,usize)> = HashMap::new();

    let (p1_wins, p2_wins) = roll_in_universe(&initial_universe, &mut memo);
    return cmp::max(p1_wins, p2_wins);
}

// Roll the dice for a round of the game
// create a new universe for each possible roll combination (27 * 27)
// end universe lines where there is a winner, and track who wins
// recursively determine the winners for each created universe
fn roll_in_universe(universe: &Universe, memo: &mut HashMap<Universe, (usize, usize)>) -> (usize, usize) {
    if let Some((p1, p2)) = memo.get(universe) {
        return (*p1, *p2);
    }    
    
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for p1_roll in dice_combos() {
        let mut u = universe.clone();
        let new_pos = calc_position(u.p1_position, p1_roll);
        u.move_p1(new_pos);
        if let Some(p1_win) = u.p1_win() {
            if p1_win {
                p1_wins += 1;
            }else {
                p2_wins += 1;
            }
            // there is a winner in this universe, no need to roll player 2
            continue;
        }
        for p2_roll in dice_combos() {
            let mut u = u.clone();
            let new_pos = calc_position(u.p2_position, p2_roll);
            u.move_p2(new_pos);
            if let Some(p1_win) = u.p1_win() {
                if p1_win {
                    p1_wins += 1;
                }else {
                    p2_wins += 1;
                }
                // there is a winner in this universe, no need for recursion
                continue;
            }
            // recursive this universe until we find winners, and how many universes they win in
            let (p1,p2) = roll_in_universe(&u, memo);
            p1_wins += p1;
            p2_wins += p2;
        }
    }
    memo.insert(universe.clone(), (p1_wins, p2_wins));
    return (p1_wins, p2_wins);
}

fn calc_position(current: i32, roll: i32) -> i32 {
    (current + roll - 1) % 10 + 1
}

// Rolling a 3 sided dice 3 times produces 27 combos
// simplify the nested loops and such by harcoding the posibilities
fn dice_combos() -> Vec<i32> {
    vec![3,
         4,4,4,
         5,5,5,5,5,5,
         6,6,6,6,6,6,6,
         7,7,7,7,7,7,
         8,8,8,
         9]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_game() {
        assert_eq!(739785, play_deterministic(4, 8));
    }

    #[test]
    fn test_dirac_uinverse() {
        assert_eq!(444356092776315, dirac_dice(4, 8));
    }
}
