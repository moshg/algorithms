use std::cmp;
use std::mem;

/// Weighted undirected Graph.
#[derive(Clone, Eq, PartialEq, Default, Debug, Hash)]
pub struct Graph {
    vertices: usize,
    weights: Box<[i64]>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        let mut graph = Graph { vertices: vertices, weights: vec![-1; vertices * vertices].into_boxed_slice() };
        for v in 0..vertices {
            graph.weights[v * vertices + v] = 0;
        }
        graph
    }

    #[inline]
    fn check_edge(&self, edge: (usize, usize)) {
        if edge.0 >= self.vertices || edge.1 >= self.vertices {
            panic!("index out of bounds: the number of vertices is {} but the edge is ({}, {})",
                   self.vertices, edge.0, edge.1);
        }
    }

    /// Returns weight of the edge if exists, or else returns -1.
    #[inline]
    pub fn weight(&self, edge: (usize, usize)) -> i64 {
        self.check_edge(edge);
        self.weights[edge.0 * self.vertices + edge.1]
    }

    /// Adds edge to the graph and returns old weight at `edge`.
    ///
    /// `weight` must be >= 0 or -1 (remove the edge)
    pub fn set_weight(&mut self, edge: (usize, usize), weight: i64) -> i64 {
        self.check_edge(edge);
        if weight < -1 {
            panic!("weight must be >= 0 or -1 but weight is {}", weight);
        }

        let old = self.weights[edge.0 * self.vertices + edge.1];
        self.weights[edge.0 * self.vertices + edge.1] = weight;
        self.weights[edge.1 * self.vertices + edge.0] = weight;
        old
    }
}

pub fn warshall_floyd(graph: Graph) -> Graph {
    let mut path = graph;
    let mut new_path = Graph::new(path.vertices);
    for k in 0..path.vertices {
        for i in 0..path.vertices {
            for j in (i + 1)..path.vertices {
                let direct = path.weight((i, j));
                let indirect = {
                    let first = path.weight((i, k));
                    let second = path.weight((k, j));
                    if first == -1 || second == -1 {
                        -1
                    } else {
                        let sum = first + second;
                        if sum < 0 {
                            panic!("overflow occurred")
                        }
                        sum
                    }
                };
                // Regard -1 as u64::MAX + 1
                new_path.set_weight((i, j), cmp::min(direct as u64, indirect as u64) as i64);
            }
        }
        mem::swap(&mut path, &mut new_path);
    }
    new_path
}

#[cfg(test)]
mod tests {
    use super::{Graph, warshall_floyd};

    #[test]
    fn test_warshall_floyd() {
        let mut graph = Graph::new(4);
        graph.set_weight((0, 1), 1);
        graph.set_weight((1, 2), 1);
        graph.set_weight((0, 2), 3);
        graph.set_weight((2, 3), 5);
        let sol = warshall_floyd(graph);
        assert_eq!(sol.weight((0, 1)), 1);
        assert_eq!(sol.weight((0, 2)), 2);
        assert_eq!(sol.weight((0, 3)), 7);
        assert_eq!(sol.weight((1, 2)), 1);
        assert_eq!(sol.weight((1, 3)), 6);
        assert_eq!(sol.weight((2, 3)), 5);
    }
}
