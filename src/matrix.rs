#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn left(&self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
    pub fn right(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MatrixIdx {
    pub row: usize,
    pub col: usize,
}
impl MatrixIdx {
    pub fn new(row: usize, col: usize) -> MatrixIdx {
        MatrixIdx { row, col }
    }
}

use std::ops::{Add, Index, IndexMut, Sub};
pub trait FromChar: Sized {
    fn try_from_char(char: &char) -> Option<Self>;
}
pub trait MatrixElement = Clone + PartialEq;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MatrixIdxOffset {
    pub rows: i64,
    pub cols: i64,
}
impl MatrixIdxOffset {
    pub const fn new(rows: i64, cols: i64) -> Self {
        MatrixIdxOffset { rows, cols }
    }
}

pub struct IdxValueIterator<'a, T: MatrixElement> {
    count: usize,
    matrix: &'a Matrix<T>,
}
impl<'a, T: MatrixElement> IdxValueIterator<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        Self { count: 0, matrix }
    }
}

impl<'a, T: MatrixElement> Iterator for IdxValueIterator<'a, T> {
    type Item = (MatrixIdx, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let count = self.count;
        let ret = self.matrix.idx_value_from_linidx(count);
        self.count += 1;
        ret
    }
}
pub struct IdxIterator<'a, T: MatrixElement> {
    count: usize,
    matrix: &'a Matrix<T>,
}

impl<'a, T: MatrixElement> IdxIterator<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        Self { count: 0, matrix }
    }
}

impl<T: MatrixElement> Iterator for IdxIterator<'_, T> {
    type Item = MatrixIdx;
    fn next(&mut self) -> Option<Self::Item> {
        let count = self.count;
        self.count += 1;
        if count <= self.matrix.data.len() {
            Some(self.matrix.idx_from_lin(count - 1))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Matrix<T>
where
    T: MatrixElement,
{
    data: Vec<T>,
    width: usize,
}
impl<'a, T> Matrix<T>
where
    T: MatrixElement,
{
    pub fn linidx(&self, idx: &MatrixIdx) -> usize {
        idx.row * self.width + idx.col
    }
    pub fn is_valid_idx(&self, idx: &MatrixIdx) -> bool {
        let h = self.height();
        let w = self.width();

        idx.col < w && idx.row < h
    }
    pub fn try_linidx(&self, idx: &MatrixIdx) -> Option<usize> {
        if self.is_valid_idx(idx) {
            Some(self.linidx(idx))
        } else {
            None
        }
    }
    pub fn get_lin(&self, linidx: usize) -> Option<&T> {
        self.data.get(linidx)
    }
    pub fn try_from_str_with(input: &str, parse: fn(&char) -> Option<T>) -> Option<Self> {
        let mut data = Vec::new();
        let mut width: Option<usize> = None;
        for line in input.lines() {
            let mut line_len = 0;
            for c in line.chars() {
                if let Some(a) = parse(&c) {
                    data.push(a);
                    line_len += 1;
                }
            }
            let width = width.get_or_insert(line_len);
            if *width != line_len {
                return None;
            }
        }
        width.map(|width| Self { data, width })
    }
    pub fn try_from_str(input: &str) -> Option<Self>
    where
        T: FromChar,
    {
        let mut data = Vec::new();
        let mut width: Option<usize> = None;
        for line in input.lines() {
            let mut line_len = 0;
            for c in line.chars() {
                if let Some(a) = T::try_from_char(&c) {
                    data.push(a);
                    line_len += 1;
                }
            }
            let width = width.get_or_insert(line_len);
            if *width != line_len {
                return None;
            }
        }
        width.map(|width| Self { data, width })
    }
    fn try_idx_from_lin(&self, linidx: usize) -> Option<MatrixIdx> {
        (linidx < self.data.len()).then(|| self.idx_from_lin(linidx))
    }
    fn idx_from_lin(&self, linidx: usize) -> MatrixIdx {
        MatrixIdx {
            row: (linidx / self.width),
            col: (linidx % self.width),
        }
    }
    fn idx_value_from_linidx(&self, linidx: usize) -> Option<(MatrixIdx, &T)> {
        self.get_lin(linidx)
            .map(|elem| (self.idx_from_lin(linidx), elem))
    }
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, idx: &MatrixIdx) -> Option<&T> {
        self.try_linidx(idx).and_then(|idx| self.get_lin(idx))
    }

    fn shape(&self) -> (usize, usize) {
        (self.height(), self.width)
    }
    pub fn get_wrapped(&self, idx: &MatrixIdx) -> &T {
        let MatrixIdx { row, col } = idx;
        let (height, width) = self.shape();

        &self[MatrixIdx {
            row: row.rem_euclid(height),
            col: col.rem_euclid(width),
        }]
    }
    pub fn idx_value_iter(&'a self) -> IdxValueIterator<'a, T> {
        IdxValueIterator::new(self)
    }
    /// Returns the indizes of this [`Matrix<T>`].
    pub fn indizes(&'a self) -> IdxIterator<'a, T> {
        IdxIterator::new(self)
    }

    pub fn find(&self, value: &T) -> Option<MatrixIdx> {
        self.data
            .iter()
            .position(|val| val == value)
            .and_then(|linidx| self.try_idx_from_lin(linidx))
    }

    pub fn get_mut(&mut self, index: &MatrixIdx) -> Option<&mut T> {
        self.try_linidx(index)
            .and_then(|index| self.data.get_mut(index))
    }

    pub fn find_all(&self, value: &T) -> Vec<MatrixIdx> {
        let mut ret = Vec::new();
        for (i, val) in self.idx_value_iter() {
            if val == value {
                ret.push(i);
            }
        }
        ret
    }
}

impl<T> Index<MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    type Output = T;

    fn index(&self, index: MatrixIdx) -> &Self::Output {
        &self.data[self.linidx(&index)]
    }
}
impl<T> Index<&MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    type Output = T;

    fn index(&self, index: &MatrixIdx) -> &Self::Output {
        &self.data[self.linidx(index)]
    }
}

impl<T> IndexMut<&MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    fn index_mut(&mut self, index: &MatrixIdx) -> &mut Self::Output {
        self.data.index_mut(self.linidx(index))
    }
}
impl<T> IndexMut<MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    fn index_mut(&mut self, index: MatrixIdx) -> &mut Self::Output {
        self.data.index_mut(self.linidx(&index))
    }
}
impl Sub<&MatrixIdx> for &MatrixIdx {
    type Output = MatrixIdxOffset;
    fn sub(self, rhs: &MatrixIdx) -> Self::Output {
        MatrixIdxOffset {
            cols: self.col as i64 - rhs.col as i64,
            rows: self.row as i64 - rhs.row as i64,
        }
    }
}
impl Sub<&MatrixIdx> for MatrixIdx {
    type Output = MatrixIdxOffset;
    fn sub(self, rhs: &MatrixIdx) -> Self::Output {
        MatrixIdxOffset {
            cols: self.col as i64 - rhs.col as i64,
            rows: self.row as i64 - rhs.row as i64,
        }
    }
}
impl Sub<MatrixIdx> for &MatrixIdx {
    type Output = MatrixIdxOffset;
    fn sub(self, rhs: MatrixIdx) -> Self::Output {
        MatrixIdxOffset {
            cols: self.col as i64 - rhs.col as i64,
            rows: self.row as i64 - rhs.row as i64,
        }
    }
}
impl Sub<MatrixIdx> for MatrixIdx {
    type Output = MatrixIdxOffset;
    fn sub(self, rhs: MatrixIdx) -> Self::Output {
        MatrixIdxOffset {
            cols: self.col as i64 - rhs.col as i64,
            rows: self.row as i64 - rhs.row as i64,
        }
    }
}
impl Sub<MatrixIdxOffset> for MatrixIdx {
    type Output = MatrixIdx;
    fn sub(self, rhs: MatrixIdxOffset) -> Self::Output {
        MatrixIdx {
            col: (self.col as i64 - rhs.cols) as usize,
            row: (self.row as i64 - rhs.rows) as usize,
        }
    }
}
impl Sub<&MatrixIdxOffset> for MatrixIdx {
    type Output = MatrixIdx;
    fn sub(self, rhs: &MatrixIdxOffset) -> Self::Output {
        MatrixIdx {
            col: (self.col as i64 - rhs.cols) as usize,
            row: (self.row as i64 - rhs.rows) as usize,
        }
    }
}
impl Add<&MatrixIdxOffset> for &MatrixIdx {
    type Output = MatrixIdx;
    fn add(self, rhs: &MatrixIdxOffset) -> Self::Output {
        MatrixIdx {
            col: (self.col as i64 + rhs.cols) as usize,
            row: (self.row as i64 + rhs.rows) as usize,
        }
    }
}
impl Add<&MatrixIdxOffset> for MatrixIdx {
    type Output = MatrixIdx;
    fn add(self, rhs: &MatrixIdxOffset) -> Self::Output {
        MatrixIdx {
            col: (self.col as i64 + rhs.cols) as usize,
            row: (self.row as i64 + rhs.rows) as usize,
        }
    }
}
impl Add<MatrixIdxOffset> for MatrixIdx {
    type Output = MatrixIdx;
    fn add(self, rhs: MatrixIdxOffset) -> Self::Output {
        MatrixIdx {
            col: (self.col as i64 + rhs.cols) as usize,
            row: (self.row as i64 + rhs.rows) as usize,
        }
    }
}
