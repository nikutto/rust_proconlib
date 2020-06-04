use super::{Weight,WeightedEdge,WeightedGraph};

use std::collections::BinaryHeap;
pub fn dijkstra(s:usize,g:&WeightedGraph) -> Vec<Weight>{
    let n = g.len();
    let mut d = vec![<Weight>::max_value();n];
    let mut heap = BinaryHeap::<(Weight,usize)>::new();
    d[s] = 0 as Weight;
    heap.push((-0 as Weight,s));
    while let Some((d_tmp,v)) = heap.pop() {
        eprintln!("{:?}",(v,-d_tmp));
        let d_now = d[v];
        if d_now< -d_tmp {
            continue;
        }
        for &WeightedEdge{to,weight} in &g[v] {
            let d_nex = d_now + weight;
            if d_nex<d[to] {
                d[to] = d_nex;
                heap.push((-d_nex,to));
            }
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use crate::graph::{build_weighted_graph,add_weighted_edge};
    use super::*;
    #[test]
    fn test_one_way_linear_graph_test() {
        let n = 10;
        let mut g:WeightedGraph = build_weighted_graph(n);
        for i in 0..(n-1) {
            add_weighted_edge(&mut g,i,i+1,2 as Weight);
        }
        let s = 3;
        let d = dijkstra(s,&g);
        assert_eq!(0,d[s]);
        assert_eq!(Weight::max_value(),d[s-3]);
        assert_eq!(Weight::max_value(),d[s-1]);
        assert_eq!(12,d[s+6]);
    }

    #[test]
    fn test_bidirectional_linear_graph() {
        let n = 10;
        let mut g:WeightedGraph = build_weighted_graph(n);
        for i in 0..(n-1) {
            add_weighted_edge(&mut g,i,i+1,2 as Weight);
            add_weighted_edge(&mut g,i+1,i,2 as Weight);
        }
        let s = 3;
        let d = dijkstra(s,&g);
        assert_eq!(0,d[s]);
        assert_eq!(6,d[s-3]);
        assert_eq!(2,d[s-1]);
        assert_eq!(12,d[s+6]);
    }

    #[test]
    fn test_simple_graph() {
        let n = 6;
        let mut g:WeightedGraph = build_weighted_graph(n);
        let s = 2;
        add_weighted_edge(&mut g,2,1,1);
        add_weighted_edge(&mut g,1,0,1);
        add_weighted_edge(&mut g,0,3,5);
        add_weighted_edge(&mut g,0,4,10);
        add_weighted_edge(&mut g,1,3,3);
        add_weighted_edge(&mut g,3,4,2);
        
        
        let d = dijkstra(s,&g);
        assert_eq!(2,d[0]);
        assert_eq!(1,d[1]);
        assert_eq!(0,d[2]);
        assert_eq!(4,d[3]);
        assert_eq!(6,d[4]);
    }

}

