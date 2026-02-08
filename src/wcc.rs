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
        .map(|i| uf.find(i)) // Svaka nit radi find() na svom delu čvorova
        .collect()
}
