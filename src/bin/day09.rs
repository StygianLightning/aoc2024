#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BlockType {
    File,
    Empty,
}

#[derive(Debug, Copy, Clone)]
struct Block {
    block_type: BlockType,
    size: u32,
}

fn main() {
    let input = std::fs::read_to_string("input/day09.txt").unwrap();
    let blocks = input
        .char_indices()
        .map(|(i, c)| Block {
            size: c.to_string().parse().unwrap(),
            block_type: if i % 2 == 0 {
                BlockType::File
            } else {
                BlockType::Empty
            },
        })
        .collect::<Vec<_>>();

    println!("{blocks:#?}");
}
