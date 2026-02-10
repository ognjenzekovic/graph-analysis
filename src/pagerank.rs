use crate::graph::Graph;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn pagerank_sequential(graph: &Graph, alpha: f64, max_iters: usize, eps: f64) -> Vec<f64> {
    let n = graph.num_nodes;
    let mut rank = vec![1.0 / n as f64; n];
    let mut new_rank = vec![0.0; n];

    let teleport = (1.0 - alpha) / n as f64;

    for iteration in 0..max_iters {
        new_rank.fill(0.0);

        for u in 0..n {
            let out_degree = graph.edges[u].len();

            if out_degree > 0 {
                let contribution = rank[u] / out_degree as f64;

                for &v in &graph.edges[u] {
                    new_rank[v] += contribution * alpha;
                }
            }
        }

        for r in &mut new_rank {
            *r += teleport;
        }

        let diff: f64 = rank
            .iter()
            .zip(&new_rank)
            .map(|(old, new)| (old - new).abs())
            .sum();

        if diff < eps {
            println!("PageRank converged after {} iterations", iteration + 1);
            return new_rank;
        }

        std::mem::swap(&mut rank, &mut new_rank);
    }

    println!("PageRank reached max iterations ({})", max_iters);

    rank
}

pub fn pagerank_parallel(
    graph: &Graph,
    alpha: f64,
    max_iters: usize,
    eps: f64,
    num_threads: usize,
) -> Vec<f64> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
        .install(|| pagerank_parallel_impl(graph, alpha, max_iters, eps))
}

#[inline]
fn f64_to_bits(val: f64) -> u64 {
    val.to_bits()
}

#[inline]
fn f64_from_bits(bits: u64) -> f64 {
    f64::from_bits(bits)
}

fn atomic_add_f64(atomic: &AtomicU64, increment: f64) {
    let mut current = atomic.load(Ordering::Acquire);

    loop {
        let current_f64 = f64_from_bits(current);
        let new_f64 = current_f64 + increment;
        let new = f64_to_bits(new_f64);
        match atomic.compare_exchange(current, new, Ordering::Release, Ordering::Acquire) {
            Ok(_) => return,
            Err(actual) => {
                current = actual;
            }
        }
    }
}

fn pagerank_parallel_impl(graph: &Graph, alpha: f64, max_iters: usize, eps: f64) -> Vec<f64> {
    let n = graph.num_nodes;
    let mut rank = vec![1.0 / n as f64; n];
    let new_rank_atomic: Vec<AtomicU64> =
        (0..n).map(|_| AtomicU64::new(f64_to_bits(0.0))).collect();

    let teleport = (1.0 - alpha) / n as f64;

    for iteration in 0..max_iters {
        new_rank_atomic.par_iter().for_each(|atomic| {
            atomic.store(f64_to_bits(0.0), Ordering::Relaxed);
        });

        (0..n).into_par_iter().for_each(|u| {
            let out_degree = graph.edges[u].len();

            if out_degree > 0 {
                let contrib = rank[u] / out_degree as f64;
                let weighted_contrib = alpha * contrib;

                for &v in &graph.edges[u] {
                    atomic_add_f64(&new_rank_atomic[v], weighted_contrib);
                }
            }
        });

        let new_rank: Vec<f64> = new_rank_atomic
            .par_iter()
            .map(|atomic| {
                let val = f64_from_bits(atomic.load(Ordering::Acquire));
                val + teleport
            })
            .collect();

        let diff: f64 = rank
            .par_iter()
            .zip(&new_rank)
            .map(|(old, new)| (old - new).abs())
            .sum();

        if diff < eps {
            println!("PageRank converged after {} iterations", iteration + 1);
            return new_rank;
        }

        rank = new_rank;
    }

    println!("PageRank reached max iterations ({})", max_iters);
    rank
}

pub fn top_nodes(ranks: &[f64], n: usize) -> Vec<(usize, f64)> {
    let mut ranked: Vec<(usize, f64)> = ranks
        .iter()
        .enumerate()
        .map(|(id, &rank)| (id, rank))
        .collect();

    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); //descending

    ranked.truncate(n);
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_simple() {
        let graph = Graph {
            num_nodes: 3,
            edges: vec![
                vec![1], // 0→1
                vec![2], // 1→2
                vec![0], // 2→0
            ],
        };

        let ranks = pagerank_sequential(&graph, 0.85, 100, 1e-6);

        assert!((ranks[0] - ranks[1]).abs() < 0.01);
        assert!((ranks[1] - ranks[2]).abs() < 0.01);

        // sum must be ~1.0
        let sum: f64 = ranks.iter().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_pagerank_star() {
        let graph = Graph {
            num_nodes: 4,
            edges: vec![
                vec![1, 2, 3], // 0→1,2,3
                vec![],        // 1
                vec![],        // 2
                vec![],        // 3
            ],
        };

        let ranks = pagerank_sequential(&graph, 0.85, 100, 1e-6);

        assert!(ranks[0] < ranks[1]);
        assert!(ranks[0] < ranks[2]);
        assert!(ranks[0] < ranks[3]);

        //around the same
        assert!((ranks[1] - ranks[2]).abs() < 0.01);
        assert!((ranks[2] - ranks[3]).abs() < 0.01);
    }

    #[test]
    fn test_top_nodes() {
        let ranks = vec![0.1, 0.4, 0.2, 0.3];
        let top = top_nodes(&ranks, 2);

        assert_eq!(top[0].0, 1); //0.4
        assert_eq!(top[1].0, 3); //0.3
    }

    #[test]
    fn test_atomic_add_f64() {
        let atomic = AtomicU64::new(f64_to_bits(0.0));

        atomic_add_f64(&atomic, 0.1);
        atomic_add_f64(&atomic, 0.2);
        atomic_add_f64(&atomic, 0.3);

        let result = f64_from_bits(atomic.load(Ordering::Acquire));
        assert!((result - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_pagerank_parallel_vs_sequential() {
        let graph = Graph {
            num_nodes: 4,
            edges: vec![vec![1, 2], vec![3], vec![3], vec![0]],
        };

        let seq = pagerank_sequential(&graph, 0.85, 100, 1e-6);
        let par = pagerank_parallel(&graph, 0.85, 100, 1e-6, 4);

        for i in 0..4 {
            assert!(
                (seq[i] - par[i]).abs() < 1e-4,
                "Node {}: seq={}, par={}",
                i,
                seq[i],
                par[i]
            );
        }
    }

    #[test]
    fn test_pagerank_par_vs_seq_large() {
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

        let seq = pagerank_sequential(&graph, 0.85, 100, 1e-8);
        let par = pagerank_parallel(&graph, 0.85, 100, 1e-8, 4);

        let sum_seq: f64 = seq.iter().sum();
        let sum_par: f64 = par.iter().sum();

        assert!((sum_seq - 1.0).abs() < 1e-6);
        assert!((sum_par - 1.0).abs() < 1e-6);

        for i in 0..10 {
            assert!((seq[i] - par[i]).abs() < 1e-4);
        }
    }
}
