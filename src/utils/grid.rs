use std::{slice::{Iter, Chunks}, ops::{IndexMut, Index, Deref}, fmt::Display, iter};

use itertools::Itertools;

pub struct Grid<T> {
    arr: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, init: Vec<T>) -> Grid<T> {
        if init.len() != width * height {
            panic!("Grid initialization size mismatch! {}x{} does not equal {}", width, height, init.len());
        }
        Grid { arr: init, width, height }
    }

    pub fn new_from<F : Fn((usize, usize)) -> T>(width: usize, height: usize, init: F) -> Grid<T> {
        let mut arr = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                arr.push(init((x, y)))
            }
        }
        Grid { arr, width, height }
    }

    pub fn with_indices(&self) -> GridIndices<T> {
        GridIndices { iter: self.arr.iter(), width: self.width, offset: 0 }
    }

    pub fn rows(&self) -> Chunks<'_, T> {
        self.arr.chunks(self.width)
    }

    pub fn get(&self, coord: (usize, usize)) -> Option<&T> {
        if coord.0 <= self.width && coord.1 <= self.height {
            Some(&self[coord])
        }
        else {
            None
        }
    }

    pub fn get_mut(&mut self, coord: (usize, usize)) -> Option<&mut T> {
        if coord.0 < self.width && coord.1 < self.height {
            Some(&mut self[coord])
        }
        else {
            None
        }
    }

}

impl<T : Copy> Grid<T> {
    pub fn map<F, U>(&self, f: F) -> Grid<U>
        where F : Fn(T) -> U
    {
        Grid { arr: self.arr.iter().map(|r| f(*r)).collect_vec(), width: self.width, height: self.height }
    }

    pub fn grow(&self, n: usize, fill: T) -> Grid<T> {
        let rows = self.arr.iter().chunks(self.width);

        let expanded_rows = rows
            .into_iter()
            .flat_map(|c| c.chain(iter::repeat(&fill).take(n * 2)))
            .map(|x| *x);

        let blank_top = iter::repeat(fill).take(self.width + n * 3);
        let blank_bottom = iter::repeat(fill).take(self.width + n);

        Grid {
            arr: blank_top.chain(expanded_rows).chain(blank_bottom).collect_vec(),
            width: self.width + n * 2,
            height: self.height + n * 2,
        }
    }

    pub fn subdivide_by<U : Copy, F, const W: usize, const H: usize>(&self, f: F) -> Grid<U>
        where F : Fn(T) -> [[U; W]; H]
    {
        let grid = self.map(f);
        Grid::new_from(self.width * W, self.height * H, |(x, y)| {
            grid[(x / W, y / H)][y % H][x % W]
        })
    }

    pub fn columns<'a>(&'a self) -> Columns<'a, T> {
        Columns::new(self)
    }

    pub fn flip_horizontal(&self) -> Self {
        let arr = self.rows()
            .flat_map(|r| r.iter().rev().map(|x| *x))
            .collect_vec();
        Grid { arr, ..*self }
    }

    pub fn transpose(&self) -> Self {
        let arr = Vec::from_iter(
            (0..self.width * self.height)
                .map(|i| self[(i / self.height, i % self.height)])
        );
        Grid { arr, width: self.height, height: self.width }
    }

}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        let width = value.find('\n').unwrap_or(value.len());
        let init: Vec<char> = value.chars()
            .filter(|c| *c != '\n')
            .collect();
        let height = init.len() / width;
        
        Grid::new(width, height, init)
    }
}

impl<T> Deref for Grid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.arr.deref()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.arr[index.0 + index.1 * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[index.0 + index.1 * self.width]
    }
}

impl<T : Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self { arr: self.arr.clone(), width: self.width, height: self.height }
    }
}

impl<T : Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.arr.len() == 0 {
            return Ok(());
        }

        fn write_row<T : Display>(f: &mut std::fmt::Formatter<'_>, row: &[T]) -> std::fmt::Result {
            row[0].fmt(f)?;
            for item in row.iter().skip(1) {
                write!(f, " ")?;
                item.fmt(f)?;
            }
            Ok(())
        }

        write_row(f, &self.arr[0..self.width])?;
        for row in self.arr.chunks(self.width).skip(1) {
            write!(f, "\n")?;
            write_row(f, row)?;
        }
        Ok(())
    }
}

pub struct GridIndices<'a, T> {
    iter: Iter<'a, T>,
    width: usize,
    offset: usize,
}

impl<'a, T> Iterator for GridIndices<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(next) => {
                let x = self.offset % self.width;
                let y = self.offset / self.width;
                self.offset += 1;
                Some(((x, y), next))
            }
        }
        
    }
}

pub struct Columns<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
}

impl<'a, T> Columns<'a, T> {
    fn new(grid: &'a Grid<T>) -> Columns<'a, T> {
        Columns { grid, x: 0 }
    }
}

impl<'a, T : Sized> Iterator for Columns<'a, T> {
    type Item = Column<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.grid.width {
            let result = Column::new(&self.grid, self.x);
            self.x += 1;
            Some(result)
        }
        else {
            None
        }
    }
}

pub struct Column<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Column<'a, T> {
    fn new(grid: &Grid<T>, x: usize) -> Column<T> {
        Column { grid, x, y: 0 }
    }
}

impl<'a, T : Sized> Iterator for Column<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.height {
            let result = &self.grid[(self.x, self.y)];
            self.y += 1;
            Some(result)
        }
        else {
            None
        }
    }
}
