use crate::graph::Graph;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by rank
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

pub fn wcc_sequential(graph: &Graph) -> Vec<usize> {
    let mut uf = UnionFind::new(graph.num_nodes);

    // make union for every edge
    for u in 0..graph.num_nodes {
        for &v in &graph.edges[u] {
            uf.union(u, v);
        }
    }

    //return ids
    (0..graph.num_nodes).map(|i| uf.find(i)).collect()
}

struct ConcurrentUnionFind {
    parent: Vec<AtomicUsize>,
}

impl ConcurrentUnionFind {
    fn new(size: usize) -> Self {
        ConcurrentUnionFind {
            parent: (0..size).map(|i| AtomicUsize::new(i)).collect(),
        }
    }

    fn find(&self, mut x: usize) -> usize {
        loop {
            let parent = self.parent[x].load(Ordering::Acquire); //ensures that everything stored with Release is available

            if parent == x {
                return x;
            }

            let grandparent = self.parent[parent].load(Ordering::Acquire);

            let _ = self.parent[x].compare_exchange(
                parent, //expected
                grandparent,
                Ordering::Release,
                Ordering::Relaxed,
            );

            x = grandparent;
        }
    }

    fn union(&self, x: usize, y: usize) {
        loop {
            let root_x = self.find(x);
            let root_y = self.find(y);

            if root_x == root_y {
                return;
            }

            let (smaller, larger) = if root_x < root_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };

            match self.parent[larger].compare_exchange(
                larger,
                smaller,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => return,
                Err(_) => continue,
            }
        }
    }
}

pub fn wcc_parallel(graph: &Graph, num_threads: usize) -> Vec<usize> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
        .install(|| wcc_parallel_impl(graph))
}

fn wcc_parallel_impl(graph: &Graph) -> Vec<usize> {
    let uf = ConcurrentUnionFind::new(graph.num_nodes);

    (0..graph.num_nodes).into_par_iter().for_each(|u| {
        for &v in &graph.edges[u] {
            uf.union(u, v);
        }
    });

    (0..graph.num_nodes)
        .into_par_iter()
        .map(|i| uf.find(i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(5);

        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);

        uf.union(0, 1);
        assert_eq!(uf.find(0), uf.find(1));

        uf.union(1, 2);
        assert_eq!(uf.find(0), uf.find(2));

        assert_ne!(uf.find(0), uf.find(3));
        assert_ne!(uf.find(0), uf.find(4));
    }

    #[test]
    fn test_wcc_simple() {
        let graph = Graph {
            num_nodes: 5,
            edges: vec![
                vec![1], // 0→1
                vec![2], // 1→2
                vec![],  // 2
                vec![4], // 3→4
                vec![],  // 4
            ],
        };

        let result = wcc_sequential(&graph);

        assert_eq!(result[0], result[1]);
        assert_eq!(result[1], result[2]);

        assert_eq!(result[3], result[4]);

        assert_ne!(result[0], result[3]);
    }

    #[test]
    fn test_wcc_all_connected() {
        let graph = Graph {
            num_nodes: 4,
            edges: vec![vec![1], vec![2], vec![3], vec![]],
        };

        let result = wcc_sequential(&graph);

        assert_eq!(result[0], result[1]);
        assert_eq!(result[1], result[2]);
        assert_eq!(result[2], result[3]);
    }

    #[test]
    fn test_wcc_all_disconnected() {
        let graph = Graph {
            num_nodes: 4,
            edges: vec![vec![], vec![], vec![], vec![]],
        };

        let result = wcc_sequential(&graph);

        assert_ne!(result[0], result[1]);
        assert_ne!(result[1], result[2]);
        assert_ne!(result[2], result[3]);
    }

    #[test]
    fn test_concurrent_union_find() {
        let uf = ConcurrentUnionFind::new(5);

        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);

        uf.union(0, 1);
        assert_eq!(uf.find(0), uf.find(1));

        uf.union(1, 2);
        assert_eq!(uf.find(0), uf.find(2));
    }

    #[test]
    fn test_wcc_parallel_vs_sequential() {
        //0→1→2, 3→4
        let graph = Graph {
            num_nodes: 5,
            edges: vec![vec![1], vec![2], vec![], vec![4], vec![]],
        };

        let seq_result = wcc_sequential(&graph);
        let par_result = wcc_parallel(&graph, 4);

        assert_eq!(
            seq_result[0] == seq_result[1],
            par_result[0] == par_result[1]
        );
        assert_eq!(
            seq_result[1] == seq_result[2],
            par_result[1] == par_result[2]
        );

        assert_eq!(
            seq_result[3] == seq_result[4],
            par_result[3] == par_result[4]
        );

        assert_ne!(par_result[0] == par_result[3], true);
    }

    #[test]
    fn test_wcc_parallel_vs_sequential_large() {
        use crate::graph_generator::generate_random;

        let path = "test_wcc_parallel.txt";
        generate_random(1000, 5000, path).unwrap();

        let graph = Graph::from_file(path).unwrap();

        let seq = wcc_sequential(&graph);
        let par = wcc_parallel(&graph, 4);

        use std::collections::HashSet;
        let seq_components: HashSet<_> = seq.iter().collect();
        let par_components: HashSet<_> = par.iter().collect();

        assert_eq!(seq_components.len(), par_components.len());

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_wcc_parallel_vs_sequential_disconnected() {
        use crate::graph_generator::generate_disconnected;

        let path = "test_wcc_parallel.txt";
        generate_disconnected(1000, 5000, 11, path).unwrap();

        let graph = Graph::from_file(path).unwrap();

        let seq = wcc_sequential(&graph);
        let par = wcc_parallel(&graph, 4);

        use std::collections::HashSet;
        let seq_components: HashSet<_> = seq.iter().collect();
        let par_components: HashSet<_> = par.iter().collect();

        assert_eq!(seq_components.len(), par_components.len());

        std::fs::remove_file(path).ok();
    }
}
