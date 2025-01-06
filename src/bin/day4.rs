use adventofcode2024::util::{load_file, MatrixIdx};
use std::ops::{Add, Index, IndexMut};
pub(crate) trait FromChar: Sized {
    fn try_from_char(char: &char) -> Option<Self>;
}
pub(crate) trait MatrixElement: FromChar + Clone + PartialEq {}

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

struct IdxValueIterator<'a, T: MatrixElement> {
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
struct IdxIterator<'a, T: MatrixElement> {
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
pub(crate) struct Matrix<T>
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
    fn linidx(&self, idx: &MatrixIdx) -> usize {
        idx.row * self.width + idx.col
    }
    fn is_valid_idx(&self, idx: &MatrixIdx) -> bool {
        let h = self.height();
        let w = self.width();

        idx.col < w && idx.row < h
    }
    fn try_linidx(&self, idx: &MatrixIdx) -> Option<usize> {
        if self.is_valid_idx(idx) {
            Some(self.linidx(idx))
        } else {
            None
        }
    }
    fn get_lin(&self, linidx: usize) -> Option<&T> {
        self.data.get(linidx)
    }
    pub fn try_from_str(input: &str) -> Option<Self> {
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
    fn idx_from_lin(&self, linidx: usize) -> MatrixIdx {
        MatrixIdx {
            row: (linidx / self.width),
            col: (linidx % self.width),
        }
    }
    fn idx_value_from_linidx(&self, linidx: usize) -> Option<(MatrixIdx, &T)> {
        self.get_lin(linidx).map(|elem| (self.idx_from_lin(linidx), elem))
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum XmasItems {
    X,
    M,
    A,
    S,
}
impl XmasItems {
    fn next(&self) -> Option<XmasItems> {
        use XmasItems::*;
        match self {
            X => Some(M),
            M => Some(A),
            A => Some(S),
            S => None,
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
impl MatrixElement for XmasItems {}

impl FromChar for XmasItems {
    fn try_from_char(char: &char) -> Option<Self> {
        match char {
            'X' => Some(XmasItems::X),
            'M' => Some(XmasItems::M),
            'A' => Some(XmasItems::A),
            'S' => Some(XmasItems::S),
            _ => None,
        }
    }
}
//#[derive(Debug)]
//struct XmasSearcher {
//    dir: Option<MatrixIdxOffset>,
//    next_item: XmasItems,
//    count: usize,
//}
//impl Default for XmasSearcher {
//    fn default() -> Self {
//        XmasSearcher {
//            dir: None,
//            next_item: XmasItems::X,
//            count: 0,
//        }
//    }
//}
//    fn traverse(
//        &mut self,
//        graph: &Matrix<XmasItems>,
//        node_id: &<Matrix<XmasItems> as Graph>::NodeIdT,
//        edge: &<Matrix<XmasItems> as Graph>::EdgeT,
//    ) -> bool;
//}
//
//trait Graph: Sized {
//    type EdgeT;
//    type NodeIdT;
//    fn next(&self, node_id: &Self::NodeIdT, edge: &Self::EdgeT) -> Option<Self::NodeIdT>;
//    fn edges(&self, node_id: &Self::NodeIdT) -> Vec<&Self::EdgeT>;
//    fn traverse<T: GraphTraverselStrategy>(&self, strategy: T) {}
//}

const DIRECTIONS: [MatrixIdxOffset; 8] = [
    MatrixIdxOffset::new(0, -1),
    MatrixIdxOffset::new(0, 1),
    MatrixIdxOffset::new(1, -1),
    MatrixIdxOffset::new(1, 0),
    MatrixIdxOffset::new(1, 1),
    MatrixIdxOffset::new(-1, -1),
    MatrixIdxOffset::new(-1, 0),
    MatrixIdxOffset::new(-1, 1),
];
//impl Graph for Matrix<XmasItems> {
//    type EdgeT = MatrixIdxOffset;
//    type NodeIdT = MatrixIdx;
//    fn next(&self, start: &Self::NodeIdT, edge: &Self::EdgeT) -> Option<Self::NodeIdT> {
//        let nidx = start + edge;
//        if self.is_valid_idx(&nidx) {
//            Some(nidx)
//        } else {
//            None
//        }
//    }
//    fn edges(&self, node_id: &Self::NodeIdT) -> Vec<&MatrixIdxOffset> {
//        let iter = DIRECTIONS.iter();
//        iter.filter_map(|d| {
//            let nidx = node_id + d;
//            if self.is_valid_idx(&nidx) {
//                Some(d)
//            } else {
//                None
//            }
//        })
//        .collect()
//    }
//}

fn check_xmas(
    matrix: &Matrix<XmasItems>,
    expected: XmasItems,
    start: MatrixIdx,
    direction: MatrixIdxOffset,
) -> bool {
    let next_index = start + direction;
    matrix.get(&next_index).is_some_and(|item| {
        item == &expected
            && item.next().is_none_or(|next_expected| {
                check_xmas(matrix, next_expected, next_index, direction)
            })
    })
}
fn part1(content: &str) -> i32 {
    let matrix = Matrix::<XmasItems>::try_from_str(content).expect("parsing into matrix failed");
    let mut solution = 0;
    for (idx, value) in matrix.idx_value_iter() {
        if value == &XmasItems::X {
            let expected = XmasItems::M;
            for direction in DIRECTIONS {
                if check_xmas(&matrix, expected, idx, direction) {
                    solution += 1;
                }
            }
        }
    }
    solution
}
fn part2(content: &str) -> i32 {
    let matrix = Matrix::<XmasItems>::try_from_str(content).expect("parsing into matrix failed");
    let mut solution = 0;
    for (idx, value) in matrix.idx_value_iter() {
        if value == &XmasItems::A {
            let expected = XmasItems::M;
            let down_diag_mas = check_xmas(
                &matrix,
                expected,
                idx + MatrixIdxOffset::new(2, 2),
                MatrixIdxOffset::new(-1, -1),
            ) || check_xmas(
                &matrix,
                expected,
                idx + MatrixIdxOffset::new(-2, -2),
                MatrixIdxOffset::new(1, 1),
            );
            let up_diag_mas = check_xmas(
                &matrix,
                expected,
                idx + MatrixIdxOffset::new(-2, 2),
                MatrixIdxOffset::new(1, -1),
            ) || check_xmas(
                &matrix,
                expected,
                idx + MatrixIdxOffset::new(2, -2),
                MatrixIdxOffset::new(-1, 1),
            );
            if up_diag_mas && down_diag_mas {
                solution += 1;
            }
        }
    }
    solution
}
fn main() {
    let content = load_file(4, 1, false).expect("failed to load input text file");
    let solution = part1(&content);
    dbg!(solution);

    let content = load_file(4, 1, false).expect("failed to load input text file");
    let solution = part2(&content);
    dbg!(solution);
}

#[cfg(test)]
mod test {
    use crate::check_xmas;

    use super::*;
    #[test]
    fn test_xmas_check() {
        // XMASS
        // MMSAA
        // ASAMM
        // SXMSX
        let matrix = Matrix::<XmasItems>::try_from_str("XMASS\nMMSAA\nASAMM\nSXMSX").unwrap();
        assert!(
            check_xmas(
                &matrix,
                XmasItems::M,
                MatrixIdx::new(0, 0),
                MatrixIdxOffset::new(0, 1)
            )
        );
        assert!(check_xmas(
            &matrix,
            XmasItems::M,
            MatrixIdx::new(0, 0),
            MatrixIdxOffset::new(1, 1)
        ));
        assert!(check_xmas(
            &matrix,
            XmasItems::M,
            MatrixIdx::new(0, 0),
            MatrixIdxOffset::new(1, 0)
        ));
        assert!(check_xmas(
            &matrix,
            XmasItems::M,
            MatrixIdx::new(3, 4),
            MatrixIdxOffset::new(-1, 0)
        ));
    }
}
