use adventofcode2024::util::{load_file, MatrixIdx};

use std::ops::{Add, Index, IndexMut};
trait FromChar: Sized {
    fn try_from_char(char: &char) -> Option<Self>;
}
pub trait MatrixElement: FromChar + Clone + PartialEq {}
trait Direction {}

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

struct IdxIterator<'a, T: MatrixElement> {
    count: usize,
    matrix: &'a Matrix<T>,
}

impl<'a, T: MatrixElement> IdxIterator<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        Self { count: 0, matrix }
    }
}

impl<'a, T: MatrixElement> Iterator for IdxIterator<'a, T> {
    type Item = MatrixIdx;
    fn next(&mut self) -> Option<Self::Item> {
        let count = self.count;
        self.count += 1;
        if count < self.matrix.data.len() {
            Some(self.matrix.idx_from_lin(count))
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
    pub fn try_from_string(input: &str) -> Option<Self> {
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
        width.and_then(|width| Some(Self { data, width }))
    }
    fn idx_from_lin(&self, linidx: usize) -> MatrixIdx {
        MatrixIdx {
            row: (linidx / self.width),
            col: (linidx % self.width),
        }
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
        &self.data[self.linidx(&index)]
    }
}

impl<T> IndexMut<&MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    fn index_mut(&mut self, index: &MatrixIdx) -> &mut Self::Output {
        self.data.index_mut(self.linidx(&index))
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
#[derive(Debug)]
struct XmasSearcher {
    dir: Option<MatrixIdxOffset>,
    next_item: XmasItems,
    count: usize,
}
impl Default for XmasSearcher {
    fn default() -> Self {
        XmasSearcher {
            dir: None,
            next_item: XmasItems::X,
            count: 0,
        }
    }
}
trait Searcher<G: Graph> {
    fn search(&mut self, graph: &G, node_id: &G::NodeIdT, edge: &G::EdgeT) -> bool;
}

trait Graph: Sized {
    type EdgeT;
    type NodeIdT;
    fn next(&self, node_id: &Self::NodeIdT, edge: &Self::EdgeT) -> Option<Self::NodeIdT>;
    fn edges(&self, node_id: &Self::NodeIdT) -> Vec<&Self::EdgeT>;
    fn depth_first_search<S: Searcher<Self>>(&self, node_id: &Self::NodeIdT, searcher: &mut S) {
        for edge in self.edges(node_id) {
            if let Some(next) = self.next(node_id, edge) {
                if searcher.search(self, node_id, edge) {
                    self.depth_first_search(&next, searcher);
                }
            }
        }
    }
}

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
impl Graph for Matrix<XmasItems> {
    type EdgeT = MatrixIdxOffset;
    type NodeIdT = MatrixIdx;
    fn next(&self, start: &Self::NodeIdT, edge: &Self::EdgeT) -> Option<Self::NodeIdT> {
        let nidx = start + edge;
        if self.is_valid_idx(&nidx) {
            Some(nidx)
        } else {
            None
        }
    }
    fn edges(&self, node_id: &Self::NodeIdT) -> Vec<&MatrixIdxOffset> {
        DIRECTIONS
            .iter()
            .filter_map(|d| {
                let nidx = node_id + d;
                if self.is_valid_idx(&nidx) {
                    Some(d)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Searcher<Matrix<XmasItems>> for XmasSearcher {
    fn search(
        &mut self,
        graph: &Matrix<XmasItems>,
        node_id: &MatrixIdx,
        edge: &MatrixIdxOffset,
    ) -> bool {
        let dir = self.dir.get_or_insert(*edge);
        if edge != dir {
            return false;
        }
        if let Some(node) = graph.get(node_id) {
            if node == &self.next_item {
                self.next_item = match node {
                    XmasItems::X => XmasItems::M,
                    XmasItems::M => XmasItems::A,
                    XmasItems::A => XmasItems::S,
                    XmasItems::S => {
                        self.count += 1;
                        return false;
                    }
                };
                return true;
            }
        }
        false
    }
}

fn main() {
    let input_text = load_file(4, 1, true).expect("failed to load input text file");
    let matrix =
        Matrix::<XmasItems>::try_from_string(&input_text).expect("parsing into matrix failed");
    // i need to fix the search algorithm it currently uses the wrong dir that is defined by the first run 
    // i think the search should be somehow differently implemented
    for idx in matrix.indizes() {
        let mut xmas_search = XmasSearcher::default();
        matrix.depth_first_search(&idx, &mut xmas_search);
        
        dbg!(idx, xmas_search);
    }
}
