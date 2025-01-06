use nalgebra::{DMatrix, DVector, Vector3};
use std::collections::{HashMap, HashSet};
pub fn parse_vec3(string: &str) -> Vector3<i64> {
    Vector3::from_iterator(
        string
            .trim()
            .split(',')
            .filter_map(|part| part.trim().parse::<i64>().ok()),
    )
}
pub fn load_file(day: i32, part: i32, test: bool) -> Result<String, std::io::Error> {
    let teststr = if test { "test_" } else { "" };

    let path = std::format!("inputs/day{day}/{teststr}input{part}.txt");
    // println!("loading data from '{}'", path);
    std::fs::read_to_string(path)
}

struct SearchState {
    node: usize,
    path: HashSet<usize>,
    distance: usize,
}

#[derive(Debug)]
pub struct Graph<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub edges: Vec<HashMap<usize, usize>>,
    num_nodes: usize,
    nodes: HashMap<T, usize>,
}
impl<T> Default for Graph<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Graph<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub fn new() -> Self {
        Self {
            edges: vec![],
            num_nodes: 0,
            nodes: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, node: T) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.nodes.entry(node) {
            e.insert(self.num_nodes);
            self.num_nodes += 1;
            self.edges.push(HashMap::new());
        }
    }
    pub fn add_edge(&mut self, from: &T, to: &T, distance: usize) {
        let nidx = self.nodes[from];
        let toidx = self.nodes[to];
        if let Some(old) = self.edges[nidx].insert(toidx, distance) {
            assert_eq!(old, distance);
        }
    }
    pub fn find_longest_path(&self, start: &T, goal: &T) -> usize {
        let start = self.nodes[start];
        let goal = self.nodes[goal];
        let mut curr = vec![SearchState {
            node: start,
            path: HashSet::new(),
            distance: 0,
        }];
        let mut maxdistance = 0;
        while !curr.is_empty() {
            let mut nexts = Vec::new();
            for state in curr.iter() {
                let SearchState {
                    node,
                    path,
                    distance,
                } = state;
                if node == &goal && distance > &maxdistance {
                    maxdistance = *distance;
                }
                let edges = &self.edges[*node];
                for next in edges.keys() {
                    if path.contains(next) {
                        continue;
                    }
                    let distance = distance + edges[next];
                    let mut path = path.clone();
                    path.insert(*next);
                    nexts.push(SearchState {
                        node: *next,
                        path,
                        distance,
                    })
                }
            }
            curr = nexts;
        }

        maxdistance
    }
    pub fn degree_matrix(&self) -> DMatrix<i64> {
        let degrees = DVector::from_iterator(
            self.edges.len(),
            self.edges.iter().map(|edge| edge.len() as i64),
        );

        DMatrix::from_diagonal(&degrees)
    }
    pub fn adjacency_matrix(&self) -> DMatrix<i64> {
        let n = self.num_nodes;
        assert_eq!(n, self.edges.len());
        let mut adjacency = DMatrix::from_element(n, n, 0i64);
        for (node, edges) in self.edges.iter().enumerate() {
            for val in edges.keys() {
                adjacency[(node, *val)] = 1;
            }
        }
        adjacency
    }
}
