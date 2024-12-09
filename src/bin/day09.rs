#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BlockType {
    File { id: u32 },
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct Block {
    block_type: BlockType,
    size: u32,
}

fn part1(blocks: &[Block]) -> u64 {
    // two pointers
    let mut blocks: Vec<Block> = blocks.to_vec();

    let mut left_idx = 0;
    let mut right_idx = blocks.len() - 1;
    let mut current_block = 0;

    let mut ret = 0;

    while left_idx <= right_idx {
        let left_block = blocks[left_idx];
        let right_block = blocks[right_idx];
        match left_block.block_type {
            BlockType::File { id } => {
                for _ in 0..left_block.size {
                    ret += id as u64 * current_block as u64;
                    current_block += 1;
                }

                left_idx += 1;
            }
            BlockType::Empty => {
                let BlockType::File { id } = right_block.block_type else {
                    right_idx -= 1;
                    continue;
                };

                let free_space = left_block.size;
                {
                    // update size total
                    let size_taken = u32::min(free_space, right_block.size);

                    for _ in 0..size_taken {
                        ret += id as u64 * current_block as u64;
                        current_block += 1;
                    }
                }

                match free_space.cmp(&right_block.size) {
                    std::cmp::Ordering::Less => {
                        // more space necessary than free space available
                        left_idx += 1;
                        blocks[right_idx].size -= free_space;
                    }
                    std::cmp::Ordering::Equal => {
                        right_idx -= 1;
                        left_idx += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        // more free space available than taken
                        right_idx -= 1;
                        blocks[left_idx].size -= right_block.size;
                    }
                }
            }
        };
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string("input/day09.txt").unwrap();

    let blocks = input
        .char_indices()
        .map(|(i, c)| Block {
            size: c.to_string().parse().unwrap(),
            block_type: if i % 2 == 0 {
                BlockType::File {
                    id: (i as u32 + 1) / 2,
                }
            } else {
                BlockType::Empty
            },
        })
        .collect::<Vec<_>>();
    let part1_res = part1(&blocks);
    println!("part 1 result: {part1_res}");
}
