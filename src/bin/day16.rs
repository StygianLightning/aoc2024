use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
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

fn part1(input: &Input) -> u64 {
    let state = State {
        position: input.start,
        facing: Direction::Left,
        cost: 0,
    };

    let mut min_cost_per_position = HashMap::new();
    min_cost_per_position.insert((state.position, state.facing), 0);

    let mut open = BinaryHeap::new();
    open.push(Reverse(state));

    while let Some(Reverse(state)) = open.pop() {
        if state.position == input.end {
            return state.cost;
        }

        let min_cost = min_cost_per_position[&(state.position, state.facing)];
        if min_cost < state.cost {
            // we've found a better path to this state in the meantime
            continue;
        }

        let mut neighbouring_states = vec![state.turn_clockwise(), state.turn_counterclockwise()];
        if let Some(n) = state.move_in(state.facing, &input.grid) {
            neighbouring_states.push(n);
        }

        for state in neighbouring_states {
            let old_cost = min_cost_per_position
                .get(&(state.position, state.facing))
                .cloned()
                .unwrap_or(u64::MAX);
            if state.cost < old_cost {
                min_cost_per_position.insert((state.position, state.facing), state.cost);
                open.push(Reverse(state));
            }
        }
    }
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    let input = parse(&input);
    println!("{input:#?}");

    let part1_res = part1(&input);
    println!("part 1 result: {part1_res}");
}
