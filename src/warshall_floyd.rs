use std::cmp;
use std::mem;

/// Weighted undirected Graph.
#[derive(Clone, Eq, PartialEq, Default, Debug, Hash)]
pub struct Graph {
    vertices: usize,
    weights: Box<[usize]>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        let mut graph = Graph { vertices: vertices, weights: vec![usize::max_value(); vertices * vertices].into_boxed_slice() };
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

    #[inline]
    pub fn weight(&self, edge: (usize, usize)) -> usize {
        self.check_edge(edge);
        self.weights[edge.0 * self.vertices + edge.1]
    }

    /// Adds edge to the graph and returns old weight at `edge`.
    pub fn set_weight(&mut self, edge: (usize, usize), weight: usize) -> usize {
        self.check_edge(edge);

        let old = self.weights[edge.0 * self.vertices + edge.1];
        self.weights[edge.0 * self.vertices + edge.1] = weight;
        self.weights[edge.1 * self.vertices + edge.0] = weight;
        old
    }
}

// FIXME: can overflow.
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
                    if first == usize::max_value() || second == usize::max_value() {
                        usize::max_value()
                    } else {
                        first + second
                    }
                };
                new_path.set_weight((i, j), cmp::min(direct, indirect));
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
