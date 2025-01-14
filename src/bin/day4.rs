use adventofcode2024::matrix::{FromChar, Matrix, MatrixIdx, MatrixIdxOffset};
use adventofcode2024::util::load_file;

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
    use adventofcode2024::matrix::Matrix;

    use crate::check_xmas;

    use super::*;
    #[test]
    fn test_xmas_check() {
        // XMASS
        // MMSAA
        // ASAMM
        // SXMSX
        let matrix = Matrix::<XmasItems>::try_from_str("XMASS\nMMSAA\nASAMM\nSXMSX").unwrap();
        assert!(check_xmas(
            &matrix,
            XmasItems::M,
            MatrixIdx::new(0, 0),
            MatrixIdxOffset::new(0, 1)
        ));
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
