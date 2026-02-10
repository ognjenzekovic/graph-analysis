use crate::graph::Graph;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn bfs_sequential(graph: &Graph, source: usize) -> Vec<i32> {
    let mut dist = vec![-1; graph.num_nodes];
    let mut queue = VecDeque::new();

    dist[source] = 0;
    queue.push_back(source);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &graph.edges[node] {
            if dist[neighbor] == -1 {
                dist[neighbor] = dist[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    dist
}

pub fn bfs_parallel(graph: &Graph, source: usize, num_threads: usize) -> Vec<i32> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
        .install(|| bfs_parallel_impl(graph, source))
}

fn bfs_parallel_impl(graph: &Graph, source: usize) -> Vec<i32> {
    let dist: Vec<AtomicI32> = (0..graph.num_nodes).map(|_| AtomicI32::new(-1)).collect();

    dist[source].store(0, Ordering::Relaxed);
    let mut current_level = vec![source];
    let mut level = 0;

    while !current_level.is_empty() {
        level += 1;

        let next_level: Vec<usize> = current_level
            .par_iter()
            .flat_map_iter(|&node| {
                graph.edges[node].iter().filter_map(|&neighbor| {
                    if dist[neighbor]
                        .compare_exchange(-1, level, Ordering::Relaxed, Ordering::Relaxed)
                        .is_ok()
                    {
                        Some(neighbor)
                    } else {
                        None
                    }
                })
            })
            .collect();

        current_level = next_level;
    }
    dist.iter().map(|d| d.load(Ordering::Relaxed)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_bfs_simple() {
        let graph = Graph {
            num_nodes: 4,
            edges: vec![
                vec![1, 3], // 0
                vec![2],    // 1
                vec![3],    // 2
                vec![],     // 3
            ],
        };

        let result = bfs_sequential(&graph, 0);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 2);
        assert_eq!(result[3], 1);
    }

    #[test]
    fn test_bfs_disconnected() {
        let graph = Graph {
            num_nodes: 4,
            edges: vec![
                vec![1], // 0
                vec![],  // 1
                vec![3], // 2
                vec![],  // 3
            ],
        };

        let result = bfs_sequential(&graph, 0);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], -1);
        assert_eq!(result[3], -1);
    }

    #[test]
    fn test_bfs_cycle() {
        let graph = Graph {
            num_nodes: 3,
            edges: vec![vec![1], vec![2], vec![0]],
        };

        let result = bfs_sequential(&graph, 0);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 2);
    }

    #[test]
    fn test_bfs_parallel_vs_sequential() {
        let graph = Graph {
            num_nodes: 5,
            edges: vec![
                vec![1, 2], // 0→1, 0→2
                vec![3],    // 1→3
                vec![3],    // 2→3
                vec![4],    // 3→4
                vec![],     // 4
            ],
        };

        let seq = bfs_sequential(&graph, 0);
        let par = bfs_parallel(&graph, 0, 4);

        for i in 0..graph.num_nodes {
            assert_eq!(seq[i], par[i], "Node {}: seq={}, par={}", i, seq[i], par[i]);
        }
    }

    #[test]
    fn test_bfs_parallel_large() {
        let graph = Graph {
            num_nodes: 10,
            edges: vec![
                vec![1, 2, 3],
                vec![4],
                vec![5],
                vec![6],
                vec![7],
                vec![8],
                vec![9],
                vec![0],
                vec![0],
                vec![0],
            ],
        };

        let seq = bfs_sequential(&graph, 0);
        let par = bfs_parallel(&graph, 0, 8);

        assert_eq!(seq, par);
    }

    #[test]
    fn test_bfs_parallel_disconnected() {
        let graph = Graph {
            num_nodes: 6,
            edges: vec![
                vec![1], // 0→1
                vec![2], // 1→2
                vec![],  // 2
                vec![4], // 3→4
                vec![5], // 4→5
                vec![],  // 5
            ],
        };

        let seq = bfs_sequential(&graph, 0);
        let par = bfs_parallel(&graph, 0, 4);

        assert_eq!(seq, par);

        assert_eq!(par[3], -1);
        assert_eq!(par[4], -1);
        assert_eq!(par[5], -1);
    }
}
