use super::{add_weighted_edge, build_weighted_graph, Weight, WeightedEdge, WeightedGraph};

pub struct Dijkstra {
    n: usize,
    g: WeightedGraph,
}

impl Dijkstra {
    pub fn new(n: usize) -> Dijkstra {
        Dijkstra {
            n: n,
            g: build_weighted_graph(n),
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, weight: Weight) {
        assert!((0..self.n).contains(&from));
        assert!((0..self.n).contains(&to));
        assert!(weight >= 0);
        add_weighted_edge(&mut self.g, from, to, weight)
    }
    pub fn solve(&self, s: usize) -> Vec<Weight> {
        assert!((0..self.n).contains(&s));
        let n = self.g.len();
        let mut d = vec![<Weight>::max_value(); n];
        let mut heap = BinaryHeap::<(Weight, usize)>::new();
        d[s] = 0 as Weight;
        heap.push((-0 as Weight, s));
        while let Some((d_tmp, v)) = heap.pop() {
            eprintln!("{:?}", (v, -d_tmp));
            let d_now = d[v];
            if d_now < -d_tmp {
                continue;
            }
            for &WeightedEdge { to, weight } in &self.g[v] {
                let d_nex = d_now + weight;
                if d_nex < d[to] {
                    d[to] = d_nex;
                    heap.push((-d_nex, to));
                }
            }
        }
        d
    }
}

use std::collections::BinaryHeap;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one_way_linear_graph_test() {
        let n = 10;
        let mut dijkstra = Dijkstra::new(n);
        for i in 0..(n - 1) {
            dijkstra.add_edge(i, i + 1, 2 as Weight);
        }
        let s = 3;
        let d = dijkstra.solve(s);
        assert_eq!(0, d[s]);
        assert_eq!(Weight::max_value(), d[s - 3]);
        assert_eq!(Weight::max_value(), d[s - 1]);
        assert_eq!(12, d[s + 6]);
    }

    #[test]
    fn test_bidirectional_linear_graph() {
        let n = 10;
        let mut dijkstra = Dijkstra::new(n);
        for i in 0..(n - 1) {
            dijkstra.add_edge(i, i + 1, 2 as Weight);
            dijkstra.add_edge(i + 1, i, 2 as Weight);
        }
        let s = 3;
        let d = dijkstra.solve(s);
        assert_eq!(0, d[s]);
        assert_eq!(6, d[s - 3]);
        assert_eq!(2, d[s - 1]);
        assert_eq!(12, d[s + 6]);
    }

    #[test]
    fn test_simple_graph() {
        let n = 6;
        let mut dijkstra = Dijkstra::new(n);
        let s = 2;
        dijkstra.add_edge(2, 1, 1);
        dijkstra.add_edge(1, 0, 1);
        dijkstra.add_edge(0, 3, 5);
        dijkstra.add_edge(0, 4, 10);
        dijkstra.add_edge(1, 3, 3);
        dijkstra.add_edge(3, 4, 2);

        let d = dijkstra.solve(s);
        assert_eq!(2, d[0]);
        assert_eq!(1, d[1]);
        assert_eq!(0, d[2]);
        assert_eq!(4, d[3]);
        assert_eq!(6, d[4]);
    }
}
