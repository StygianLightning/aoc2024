use aoc2024::{
    grid::Grid,
    index2::{uidx2, Index2, UIndex2},
};

#[derive(Debug)]
struct Robot {
    position: Index2,
    velocity: Index2,
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            fn extract(text: &str) -> Index2 {
                let mut split = text.split("=").skip(1).next().unwrap().split(",");
                Index2::new(
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            }

            let position = extract(split.next().unwrap());
            let velocity = extract(split.next().unwrap());
            Robot { position, velocity }
        })
        .collect()
}

fn robot_position_after_iterations(
    robot: &Robot,
    num_iterations: u32,
    map_size: UIndex2,
) -> UIndex2 {
    let raw_position = robot.position + num_iterations as i32 * robot.velocity;
    let x = ((raw_position.x % map_size.x as i32) + map_size.x as i32) as u32 % map_size.x;
    let y = ((raw_position.y % map_size.y as i32) + map_size.y as i32) as u32 % map_size.y;
    uidx2(x, y)
}

fn part1(robots: &[Robot], num_iterations: u32, map_size: UIndex2) -> u32 {
    let mut grid = Grid::<u32>::new_with_default(map_size);

    for robot in robots {
        let position_after = robot_position_after_iterations(robot, num_iterations, map_size);
        grid[position_after] += 1;
    }

    // quadrants
    let half_width = map_size.x / 2;
    let half_height = map_size.y / 2;

    let mut left_width_start = 0;
    let mut right_width_start = map_size.x - half_width;
    let mut top_height_start = 0;
    let mut bottom_height_start = map_size.y - half_height;
    let quadrants = vec![
        // top left
        (
            uidx2(left_width_start, top_height_start),
            uidx2(half_width, half_height),
        ),
        // bottom left
        (
            uidx2(left_width_start, bottom_height_start),
            uidx2(half_width, map_size.y),
        ),
        // top right
        (
            uidx2(right_width_start, top_height_start),
            uidx2(map_size.x, half_height),
        ),
        // bottom right
        (
            uidx2(right_width_start, bottom_height_start),
            uidx2(map_size.x, map_size.y),
        ),
    ];

    let mut robots_per_quadrant = vec![0, 0, 0, 0];
    for (quadrant, (start, end)) in quadrants.iter().enumerate() {
        for x in start.x..end.x {
            for y in start.y..end.y {
                let idx = uidx2(x, y);
                robots_per_quadrant[quadrant] += grid[idx];
            }
        }
    }

    robots_per_quadrant
        .iter()
        .cloned()
        .reduce(|a, b| a * b)
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    let robots = parse(&input);

    let map_size = uidx2(101, 103);
    let part1_res = part1(&robots, 100, map_size);
    println!("part 1 result: {part1_res}");

    let part2_res = part2(&robots, map_size);
    println!("part 2 result: {part2_res}");
}

fn part2(robots: &[Robot], map_size: UIndex2) -> u32 {
    let mut grid = Grid::<u32>::new_with_default(map_size);
    let mut max_connected = 0;

    for num_iterations in 0..100_000 {
        for robot in robots {
            let position_after = robot_position_after_iterations(robot, num_iterations, map_size);
            grid[position_after] += 1;
        }

        // check if tree
        let mut print_potential_tree = false;

        const CONNECTED_TREE_THRESHOLD: u32 = 25;

        for y in 0..grid.dimension().y {
            let mut connected = 0;
            for x in 0..grid.dimension().x {
                let idx = uidx2(x, y);
                let value = grid[idx];
                if value > 0 {
                    connected += 1;
                    max_connected = u32::max(max_connected, connected);

                    if connected >= CONNECTED_TREE_THRESHOLD {
                        print_potential_tree = true;
                    }
                } else {
                    connected = 0;
                }
            }
        }

        if print_potential_tree {
            for y in 0..grid.dimension().y {
                for x in 0..grid.dimension().x {
                    let value = grid[uidx2(x, y)];
                    if value == 0 {
                        print!(".");
                    } else {
                        print!("{}", value);
                    }
                }
                println!();
            }
            println!();

            return num_iterations;
        }
        grid.reset_to_default();
    }
    panic!("no tree found");
}
