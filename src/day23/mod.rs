/*
Day 23: Amphipod

Solve a logic puzzle for a group of amphipods.
A burrow is a hallway with 4 rooms, and different types of amphipods (A,B,C,D) in each room.
Move the amphipods so that all the As are in the first room, Bs in the second, etc.

Amphipods never stop in the hallway space immediately outside a room.
Amphipods in the hallway can only move to their correct destination room
Amphipods cannot move into their destination room unless no other type of Amphipod is inside
If an amphipod is in the correct destination room, it can only leave that room
if a different type of amphipod is also in that room.
Moving costs are different for each amphipod, A=1, B=10, C=100, D=1000

Part 1: given a starting puzzle with 2 spaces in each room, what is the lowest energy cost solution?

Part 2: given a puzzle with 4 spaces in each room, what is the lowest energy cost solution?
*/

use std::collections::HashSet;
use std::fmt;
use std::cmp;

// Each amphipod type represented as an enum
#[derive(Clone, Eq, PartialEq)]
pub enum Amphipod {
    A, B, C, D
}

impl Amphipod {
    fn energy(&self) -> i32 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    // Create an iterator over each Amphipod enum value
    fn each() -> Box<dyn Iterator<Item=Amphipod>> {
        Box::new(vec![Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D].into_iter())
    }
}


impl fmt::Debug for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Amphipod::A => write!(f, "A"),
            Amphipod::B => write!(f, "B"),
            Amphipod::C => write!(f, "C"),
            Amphipod::D => write!(f, "D")
        }
    }
}

const ENTRY_SPACES: [usize; 4] = [2,4,6,8];

// The Burrow struct represents the state of the puzzle
#[derive(Clone)]
pub struct Burrow {
    hallway: Vec<Option<Amphipod>>,
    rooms: Vec<Vec<Option<Amphipod>>>
}

impl Burrow {

    // Returns the index of the hallway space for the destination room of the given aphipod
    fn get_entry_space(amphipod: &Amphipod) -> usize {
        match amphipod {
            Amphipod::A => ENTRY_SPACES[0],
            Amphipod::B => ENTRY_SPACES[1],
            Amphipod::C => ENTRY_SPACES[2],
            Amphipod::D => ENTRY_SPACES[3],
        }
    }

    // returns the index representing the destination room of the given amphipod
    fn room_index(amphipod: &Amphipod) -> usize {
        match amphipod {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    // create an new burrow board setup
    // param is a 2d vector of Amphipods that represents the starting position inside each room
    // ex. initial[1][0] == Amphipod::D means that in the 2nd room (the B room), the first space has a D
    fn new(initial: Vec<Vec<Amphipod>>) -> Burrow {
        Burrow {
            hallway: vec![None; 11],
            rooms: initial.into_iter()
                .map(|room| room.into_iter()
                    .map(|a| Some(a))
                    .collect()
                )
                .collect()
        }
    }

    // true if the burrow is complete and all amphipods are in the correct room
    fn is_complete(&self) -> bool {
        if self.hallway.iter().any(|space| space.is_some()) {
            return false;
        }
        if !self.rooms[0].iter().all(|space| space == &Some(Amphipod::A)) {
            return false;
        }
        if !self.rooms[1].iter().all(|space| space == &Some(Amphipod::B)) {
            return false;
        }
        if !self.rooms[2].iter().all(|space| space == &Some(Amphipod::C)) {
            return false;
        }
        if !self.rooms[3].iter().all(|space| space == &Some(Amphipod::D)) {
            return false;
        }
        return true;
    }

    // Return the destination room of the given amphipod
    // the room is a vector with each value representing what occupies each space in the room:
    // None for empty, Some(_) for the Amphipod in that space
    fn get_room(&self, amphipod: &Amphipod) -> &Vec<Option<Amphipod>> {
        match amphipod {
            Amphipod::A => &self.rooms[0],
            Amphipod::B => &self.rooms[1],
            Amphipod::C => &self.rooms[2],
            Amphipod::D => &self.rooms[3],
        }
    }

    // Check if the burrow is in a known unsolvable state
    fn is_invalid(&self) -> bool {
        // If an A is in the hallway blocking off the rest of the rooms
        // and a non A is in the A room, and there are no free spaces to the left
        // then we are stuck and cannot solve
        if self.hallway[1].is_some() 
                && self.hallway[3] == Some(Amphipod::A)
                && self.rooms[0].iter().any(|space| space.is_some() && space != &Some(Amphipod::A)) {
            return true;
        }

        // If a D is in the hallway blocking the other three rooms
        // and a non-D is in the D room, and there is no space to the right
        // then we are stuck and cannot solve
        if self.hallway[9].is_some()
                && self.hallway[7] == Some(Amphipod::D)
                && self.rooms[3].iter().any(|space| space.is_some() && space != &Some(Amphipod::D)) {
        }

        return false;
    }

    // If all amphipods could immidiately move to the correct room,
    // regardless of obstacles or rules, how much energy would it take?
    // Use as a heuristic to evaluate the board state
    fn naive_solve_energy(&self) -> i32 {
        let mut cost = 0;
        for i in 0..self.rooms[0].len() {
            cost += match self.rooms[0][i] {
                None => 0,
                Some(Amphipod::A) => 0,
                Some(Amphipod::B) => (i + 1 + 3) as i32 * Amphipod::B.energy(),
                Some(Amphipod::C) => (i + 1 + 5) as i32 * Amphipod::C.energy(),
                Some(Amphipod::D) => (i + 1 + 7) as i32 * Amphipod::D.energy(),
            }
        }
        for i in 0..self.rooms[1].len() {
            cost += match self.rooms[1][i] {
                None => 0,
                Some(Amphipod::A) => (i + 1 + 3) as i32 * Amphipod::A.energy(),
                Some(Amphipod::B) => 0,
                Some(Amphipod::C) => (i + 1 + 3) as i32 * Amphipod::C.energy(),
                Some(Amphipod::D) => (i + 1 + 5) as i32 * Amphipod::D.energy(),
            }
        }
        for i in 0..self.rooms[2].len() {
            cost += match self.rooms[2][i] {
                None => 0,
                Some(Amphipod::A) => (i + 1 + 5) as i32 * Amphipod::A.energy(),
                Some(Amphipod::B) => (i + 1 + 3) as i32 * Amphipod::B.energy(),
                Some(Amphipod::C) => 0,
                Some(Amphipod::D) => (i + 1 + 3) as i32 * Amphipod::D.energy(),
            }
        }
        for i in 0..self.rooms[3].len() {
            cost += match self.rooms[3][i] {
                None => 0,
                Some(Amphipod::A) => (i + 1 + 7) as i32 * Amphipod::A.energy(),
                Some(Amphipod::B) => (i + 1 + 5) as i32 * Amphipod::B.energy(),
                Some(Amphipod::C) => (i + 1 + 3) as i32 * Amphipod::C.energy(),
                Some(Amphipod::D) => 0,
            }
        }

        for i in 0..self.hallway.len() {
            cost += match self.hallway[i] {
                None => 0,
                Some(Amphipod::A) => ((i as i32 - 2).abs() + 1) * Amphipod::A.energy(),
                Some(Amphipod::B) => ((i as i32 - 4).abs() + 1) * Amphipod::B.energy(),
                Some(Amphipod::C) => ((i as i32 - 6).abs() + 1) * Amphipod::C.energy(),
                Some(Amphipod::D) => ((i as i32 - 8).abs() + 1) * Amphipod::D.energy()
            }
        }

        return cost;
    }
    
}

// Some helpers to print out the burrow into a human readable format
impl fmt::Debug for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = self.hallway.iter().map(|space| format_space(space)).collect::<Vec<_>>().join("");
        let mut lines: Vec<String> = Vec::new();
        for i in 0..self.rooms[0].len() {
            lines.push(format!("  #{:?}#{:?}#{:?}#{:?}#", 
                format_space(&self.rooms[0][i]),
                format_space(&self.rooms[1][i]),
                format_space(&self.rooms[2][i]),
                format_space(&self.rooms[3][i])
            ).replace("\"", ""));
        }
        let rooms_str = lines.join("\n");
        write!(f, "#############\n#{}#\n{}", h, rooms_str)
    }
}

fn format_space(space: &Option<Amphipod>) -> String {
    match space {
        None => ".".to_string(),
        Some(a) => format!("{:?}", a).to_string()
    }
}

// Parts 1 and 2
// find the lowest energy solution. Takes around 25 seconds for each puzzle.
// Use a DFS with pruning to evaluate all possible legal moves
pub fn lowest_energy_solution(burrow: &Burrow) -> i32 {
    let mut costs: HashSet<i32> = HashSet::new();
    next_move(burrow, 0, &mut costs);
    costs.into_iter().min().unwrap()
}

// Main recursive driver function
// evaluates all moves from the given burrow state, but recursively depth first
fn next_move(burrow: &Burrow, energy: i32, completed_cost: &mut HashSet<i32>) {
    if let Some(min) = completed_cost.iter().min() {
        // naively estimate how much energy it would take to solve from the current state
        // if we already have a solution with less energy, we can stop this DFS path now
        if *min <= energy + burrow.naive_solve_energy() {
            return;
        }
    }

    // Check for some known unsolvable states
    if burrow.is_invalid() {
        return;
    }

    // check for valid moves for amphipods in the hallway
    for i in 0..burrow.hallway.len() {
        if let Some(amphipod) = &burrow.hallway[i] {
            let destination_room = burrow.get_room(amphipod);
            // can only move into the destination room if the room has no other type of amphipod
            if destination_room.iter().any(|space| space.is_some() && space.as_ref() != Some(amphipod)) {
                continue;
            }

            // must be able to reach the destination room without any obstacles in the way
            let entryway = Burrow::get_entry_space(amphipod);
            let mut clear_path = true;
            let start;
            let end;
            if i < entryway {
                start = i+1;
                end = entryway;
            }
            else {
                start = entryway;
                end = i-1;
            }
            for space in start..=end {
                if let Some(_) = burrow.hallway[space] {
                    clear_path = false;
                }
            }
            if !clear_path {
                continue;
            }

            // Move the current amphipod into the lowest open space in the destination room
            let mut farthest_open = 0;
            for s in (0..destination_room.len()).rev() {
                if destination_room[s].is_none() {
                    farthest_open = s;
                    break;
                }
            }

            // calculate movement cost
            let move_cost = ((i as i32 - entryway as i32).abs() + farthest_open as i32 + 1) * amphipod.energy();
            if let Some(min) = completed_cost.iter().min() {
                if min <= &(energy + move_cost) {
                    // we already have a better solution to the problem, stop here
                    return;
                }
            }

            // Clone the burrow and make the moves
            let mut next_burrow = burrow.clone();
            next_burrow.hallway[i] = None;
            next_burrow.rooms[Burrow::room_index(amphipod)][farthest_open] = Some(amphipod.clone());

            if next_burrow.is_complete() {
                completed_cost.insert(energy + move_cost);
                return;
            }
            next_move(&next_burrow, energy + move_cost, completed_cost);
        }
    }

    // Check amphipods in rooms for moves
    for amphipod_type in Amphipod::each() {
        let room = burrow.get_room(&amphipod_type);
        for space in 0..room.len() {
            if let Some(amphipod) = &room[space] {
                // Amphipod is in the correct room and no wrong amphipod is also in the room
                // so we skip this one, it has no legal moves
                if amphipod == &amphipod_type && !room.iter().any(|s| s.is_some() && s.as_ref() != Some(&amphipod_type)) {
                    continue;
                }

                // If the amphipod is blocked from exiting the room, skip it
                let mut clear_exit = true;
                for i in 0..space {
                    if let Some(_) = &room[i] {
                        clear_exit = false;
                        break;
                    }
                }
                if !clear_exit {
                    continue;
                }

                let entryway = Burrow::get_entry_space(&amphipod_type);

                // First, check if this amphipod can move to its correct room
                let destination_room = burrow.get_room(amphipod);
                // can only move into the destination room if the room has no other type of amphipod
                if destination_room.iter().all(|space| space.is_none() || space.as_ref() == Some(amphipod)) {
                    let destination_entry = Burrow::get_entry_space(amphipod);
                    let mut hall_clear = true;
                    for hall_path in cmp::min(entryway,destination_entry)..cmp::max(entryway,destination_entry) {
                        if burrow.hallway[hall_path].is_some() {
                            hall_clear = false;
                        }
                    }

                    // if the hallway between rooms is clear, move to the farthest open space in the correct room
                    if destination_entry != entryway && hall_clear {
                        let mut farthest_open = 0;
                        for s in (0..destination_room.len()).rev() {
                            if destination_room[s].is_none() {
                                farthest_open = s;
                                break;
                            }
                        }
                        // cost
                        let move_cost = (space as i32 + 1 + (entryway as i32 - destination_entry as i32).abs() + farthest_open as i32 + 1) * amphipod.energy();

                        // Clone the burrow and make the moves
                        let mut next_burrow = burrow.clone();
                        next_burrow.rooms[Burrow::room_index(&amphipod_type)][space] = None;
                        next_burrow.rooms[Burrow::room_index(amphipod)][farthest_open] = Some(amphipod.clone());
                        if next_burrow.is_complete() {
                            println!("Completed! {}", energy + move_cost);
                            completed_cost.insert(energy + move_cost);
                            return;
                        }
                        next_move(&next_burrow, energy + move_cost, completed_cost);
                        // no need to enumerate other possible moves
                        // a move to the correct final room is always the best move from this burrow state
                        continue;
                    }
                }

                // Now evaluate all possible moves into the hallway
                // Go left until we are blocked. Recurse for each valid movement
                for i in (0..entryway).rev() {
                    if let Some(_) = burrow.hallway[i] {
                        break;
                    }
                    // cannot land on an entry space
                    if ENTRY_SPACES.contains(&i) {
                        continue;
                    }

                    let cost = (entryway - i + space + 1) as i32 * amphipod.energy();
                    let mut next_burrow = burrow.clone();
                    next_burrow.rooms[Burrow::room_index(&amphipod_type)][space] = None;
                    next_burrow.hallway[i] = Some(amphipod.clone());
                    next_move(&next_burrow, energy + cost, completed_cost);
                }

                // Go right until we are blocked. Recurse for each valid movement
                for i in entryway+1..burrow.hallway.len() {
                    if let Some(_) = burrow.hallway[i] {
                        break;
                    }
                    // cannot land on an entry space
                    if ENTRY_SPACES.contains(&i) {
                        continue;
                    }

                    let cost = (i - entryway + space + 1) as i32 * amphipod.energy();
                    let mut next_burrow = burrow.clone();
                    next_burrow.rooms[Burrow::room_index(&amphipod_type)][space] = None;
                    next_burrow.hallway[i] = Some(amphipod.clone());
                    next_move(&next_burrow, energy + cost, completed_cost);
                }
            }
        }
    }
}

pub fn part_1_start() -> Burrow {
    let init = vec![vec![Amphipod::B, Amphipod::B],
        vec![Amphipod::A, Amphipod::C],
        vec![Amphipod::A, Amphipod::D],
        vec![Amphipod::D, Amphipod::C]];

    Burrow::new(init)
}

pub fn part_2_start() -> Burrow {
    let init = vec![vec![Amphipod::B, Amphipod::D, Amphipod::D, Amphipod::B],
        vec![Amphipod::A, Amphipod::C, Amphipod::B, Amphipod::C],
        vec![Amphipod::A, Amphipod::B, Amphipod::A, Amphipod::D],
        vec![Amphipod::D, Amphipod::A, Amphipod::C, Amphipod::C]];

    Burrow::new(init)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_cost_2_room() {
        let init = vec![vec![Amphipod::B, Amphipod::A],
            vec![Amphipod::C, Amphipod::D],
            vec![Amphipod::B, Amphipod::C],
            vec![Amphipod::D, Amphipod::A]];

        let burrow = Burrow::new(init);
        assert_eq!(12521, lowest_energy_solution(&burrow));
    }

    #[test]
    fn test_lowest_cost_4_room() {
        let init = vec![vec![Amphipod::B, Amphipod::D, Amphipod::D, Amphipod::A],
            vec![Amphipod::C, Amphipod::C, Amphipod::B, Amphipod::D],
            vec![Amphipod::B, Amphipod::B, Amphipod::A, Amphipod::C],
            vec![Amphipod::D, Amphipod::A, Amphipod::C, Amphipod::A]];

        let burrow = Burrow::new(init);
        assert_eq!(44169, lowest_energy_solution(&burrow));
    }
}

