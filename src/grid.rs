use super::index2::{uidx2, UIndex2};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid<T> {
    data: Vec<T>,
    dimension: UIndex2,
}

impl<T: Default> Grid<T> {
    pub fn new_with_default(size: UIndex2) -> Self {
        let mut data = Vec::new();
        data.resize_with(size.size() as usize, || Default::default());
        Self {
            dimension: size,
            data,
        }
    }

    pub fn reset_to_default(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = Default::default()
        }
    }
}

impl<T> Grid<T> {
    pub fn new_with_provider<F: FnMut() -> T>(size: UIndex2, f: F) -> Self {
        let mut data = Vec::new();
        data.resize_with(size.size() as usize, f);
        Self {
            dimension: size,
            data,
        }
    }

    pub fn dimension(&self) -> UIndex2 {
        self.dimension
    }

    pub fn get(&self, index: UIndex2) -> Option<&T> {
        try_linearize(index, self.dimension).and_then(|idx| self.data.get(idx as usize))
    }

    pub fn get_mut(&mut self, index: UIndex2) -> Option<&mut T> {
        try_linearize(index, self.dimension).and_then(move |idx| self.data.get_mut(idx as usize))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

impl<T> Index<UIndex2> for Grid<T> {
    type Output = T;

    fn index(&self, index: UIndex2) -> &Self::Output {
        &self.data[linearize(index, self.dimension) as usize]
    }
}

impl<T> IndexMut<UIndex2> for Grid<T> {
    fn index_mut(&mut self, index: UIndex2) -> &mut Self::Output {
        &mut self.data[linearize(index, self.dimension) as usize]
    }
}

pub fn linearize(idx: UIndex2, dimension: UIndex2) -> u32 {
    idx.y * dimension.x + idx.x
}

pub fn structurize(linear_idx: u32, dimension: UIndex2) -> UIndex2 {
    uidx2(linear_idx % dimension.x, linear_idx / dimension.x)
}

pub fn try_linearize(idx: UIndex2, dimension: UIndex2) -> Option<u32> {
    if idx.x >= dimension.x || idx.y >= dimension.y {
        None
    } else {
        Some(linearize(idx, dimension))
    }
}

pub fn try_structurize(linear_idx: u32, dimension: UIndex2) -> Option<UIndex2> {
    let structured = structurize(linear_idx, dimension);
    if structured.x >= dimension.x || structured.y >= dimension.y {
        None
    } else {
        Some(structured)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_usage() {
        let mut grid: Grid<i32> = Grid::new_with_default(uidx2(13, 7));

        assert_eq!(grid[uidx2(0, 0)], 0);
        grid[uidx2(0, 0)] = 10;
        assert_eq!(grid[uidx2(0, 0)], 10);

        assert!(grid.get(uidx2(17, 0)).is_none());
    }

    #[test]
    fn grid_clear() {
        let mut grid: Grid<i32> = Grid::new_with_default(uidx2(13, 7));

        grid[uidx2(0, 0)] = 10;

        grid.reset_to_default();
        //clearing the grid resets all the values to Default::default()
        assert_eq!(grid.get(uidx2(0, 0)), Some(&0));
    }
}
