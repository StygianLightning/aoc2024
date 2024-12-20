use crate::{
    grid::Grid,
    index2::{uidx2, Index2, UIndex2},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: &[Self] = &[Self::Up, Self::Down, Self::Left, Self::Right];

    pub fn to_index2(self) -> Index2 {
        match self {
            Direction::Up => Index2 { x: 0, y: -1 },
            Direction::Down => Index2 { x: 0, y: 1 },
            Direction::Left => Index2 { x: -1, y: 0 },
            Direction::Right => Index2 { x: 1, y: 0 },
        }
    }

    pub fn turn_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn invert(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn get_neighbor<T>(self, idx: UIndex2, grid: &Grid<T>) -> Option<UIndex2> {
        let offset = self.to_index2();
        let target = idx.to_index2() + offset;

        let Some(idx) = target.to_index2() else {
            return None;
        };
        if grid.get(idx).is_some() {
            Some(idx)
        } else {
            None
        }
    }

    pub fn offset_index(&self, idx: Index2) -> Index2 {
        idx + self.to_index2()
    }
}
