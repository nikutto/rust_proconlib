pub mod dijkstra;

type Weight = i64;

#[derive(Clone,Copy)]
pub struct WeightedEdge{
    pub to : usize,
    pub weight : Weight,
}

pub type WeightEdges = Vec<WeightedEdge>;
pub type WeightedGraph = Vec<WeightEdges>;

pub fn build_weighted_graph(n:usize) -> WeightedGraph{
    vec![<WeightEdges>::new();n]
}

pub fn add_weighted_edge(g:&mut WeightedGraph,from:usize,to:usize,weight:Weight){
    g[from].push(WeightedEdge{to,weight})
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_build_weighted_graph(){
        let n = 3;
        let g = build_weighted_graph(n);
        assert_eq!(3,g.len());
        for i in 0..n {
            assert_eq!(0,g[i].len());
        }
    }

    #[test]
    fn test_add_weighted_edge(){
        let n = 3;
        let mut g = build_weighted_graph(n);
        add_weighted_edge(&mut g,0,1,2);
        add_weighted_edge(&mut g,0,2,3);
        add_weighted_edge(&mut g,1,0,4);
        
        assert_eq!(g[0][0].to,1);
        assert_eq!(g[0][0].weight,2);
        assert_eq!(g[0][1].to,2);
        assert_eq!(g[0][1].weight,3);
        assert_eq!(g[1][0].to,0);
        assert_eq!(g[1][0].weight,4);
    }
}

