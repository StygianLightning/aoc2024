use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    vec,
};

use aoc2024::{
    direction::Direction,
    grid::Grid,
    index2::{uidx2, UIndex2},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

#[derive(Debug)]
struct Input {
    start: UIndex2,
    end: UIndex2,
    grid: Grid<Tile>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    position: UIndex2,
    facing: Direction,
    cost: u64,
}

impl State {
    fn turn_clockwise(self) -> Self {
        Self {
            position: self.position,
            facing: self.facing.turn_clockwise(),
            cost: self.cost + COST_ROTATE,
        }
    }
    fn turn_counterclockwise(self) -> Self {
        Self {
            position: self.position,
            facing: self.facing.invert().turn_clockwise(),
            cost: self.cost + COST_ROTATE,
        }
    }

    fn move_in(self, direction: Direction, grid: &Grid<Tile>) -> Option<Self> {
        if let Some(position) = direction.get_neighbor(self.position, grid) {
            if grid[position] == Tile::Empty {
                return Some(Self {
                    position,
                    facing: self.facing,
                    cost: self.cost + COST_MOVE,
                });
            }
        }
        None
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.position.x.cmp(&other.position.x))
            .then(self.position.y.cmp(&other.position.y))
            .then(self.facing.cmp(&other.facing))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.lines().take_while(|l| !l.trim().is_empty()).count() as u32;
    let size = uidx2(width, height);

    let mut grid = Grid::new_with_default(size);
    let mut start = uidx2(0, 0);
    let mut end = uidx2(0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let idx = uidx2(x as u32, y as u32);
            grid[idx] = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    start = idx;

                    Tile::Empty
                }
                'E' => {
                    end = idx;
                    Tile::Empty
                }
                _ => panic!("unknown tile {c}"),
            };
        }
    }

    Input { grid, start, end }
}

const COST_ROTATE: u64 = 1000;
const COST_MOVE: u64 = 1;

fn analyze(input: &Input) -> (u64, u64) {
    let state = State {
        position: input.start,
        facing: Direction::Left,
        cost: 0,
    };

    let mut min_cost_per_position = HashMap::new();
    min_cost_per_position.insert((state.position, state.facing), 0);

    let mut open = BinaryHeap::new();
    open.push(Reverse(state));

    // for each state, save ALL preceding positions with the minimum cost
    let mut min_cost_predecessors = HashMap::new();

    let mut best_state = state;
    while let Some(Reverse(current_state)) = open.pop() {
        if current_state.position == input.end {
            best_state = current_state;
            break;
        }

        let min_cost = min_cost_per_position[&(current_state.position, current_state.facing)];
        if min_cost < current_state.cost {
            // we've found a better path to this state in the meantime
            continue;
        }

        let mut neighbouring_states = vec![
            current_state.turn_clockwise(),
            current_state.turn_counterclockwise(),
        ];
        if let Some(n) = current_state.move_in(current_state.facing, &input.grid) {
            neighbouring_states.push(n);
        }

        for new_state in neighbouring_states {
            let old_cost = min_cost_per_position
                .get(&(new_state.position, new_state.facing))
                .cloned()
                .unwrap_or(u64::MAX);
            if new_state.cost < old_cost {
                min_cost_per_position
                    .insert((new_state.position, new_state.facing), new_state.cost);
                open.push(Reverse(new_state));
                min_cost_predecessors
                    .insert((new_state.position, new_state.facing), vec![current_state]);
            }
            if new_state.cost == old_cost {
                // going from the current state to the new state is an alternate path with the same total cost
                min_cost_predecessors
                    .get_mut(&(new_state.position, new_state.facing))
                    .unwrap()
                    .push(current_state);
            }
        }
    }

    if best_state.position != input.end {
        // couldn't reach the target
        panic!("No path to target exists");
    }

    let mut positions_on_best_path = HashSet::new();
    positions_on_best_path.insert(best_state.position);

    let mut open = vec![];
    for direction in Direction::ALL {
        let cost = min_cost_per_position
            .get(&(best_state.position, *direction))
            .cloned()
            .unwrap_or(u64::MAX);
        if cost == best_state.cost {
            open.push((best_state.position, *direction));
        }
    }

    while let Some((position, facing)) = open.pop() {
        if let Some(predecessors) = min_cost_predecessors.get(&(position, facing)) {
            for pred in predecessors {
                positions_on_best_path.insert(pred.position);
                open.push((pred.position, pred.facing));
            }
        }
    }

    (best_state.cost, positions_on_best_path.len() as _)
}

fn main() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    let input = parse(&input);

    let (part1, part2) = analyze(&input);
    println!("part 1 result: {part1}");
    println!("part 2 result: {part2}");
}
