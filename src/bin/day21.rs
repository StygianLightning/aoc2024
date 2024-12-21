use aoc2024::{grid::Grid, index2::uidx2};

#[derive(Debug)]
struct Keypad {
    grid: Grid<Option<char>>,
}

impl Keypad {
    fn directional() -> Self {
        let mut grid = Grid::new_with_default(uidx2(3, 2));
        // None, ^, A
        // (0, 0) stays None
        grid[uidx2(1, 0)] = Some('^');
        grid[uidx2(2, 0)] = Some('A');

        // <, v, >
        grid[uidx2(0, 1)] = Some('<');
        grid[uidx2(1, 1)] = Some('v');
        grid[uidx2(2, 1)] = Some('>');

        Self { grid }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day21.txt").unwrap();
    let input = std::fs::read_to_string("input/test_day21.txt").unwrap();

    let codes = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            (!line.is_empty()).then_some(line)
        })
        .collect::<Vec<_>>();

    let int_codes = codes
        .iter()
        .map(|l| {
            l.chars()
                .skip_while(|c| c.is_alphabetic() || *c == '0')
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();
    println!("{int_codes:?}");
}
