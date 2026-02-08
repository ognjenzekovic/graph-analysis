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

//faster but more memory usage
#[allow(dead_code)]
fn pagerank_parallel_buffered(graph: &Graph, alpha: f64, max_iters: usize, eps: f64) -> Vec<f64> {
    let n = graph.num_nodes;
    let mut rank = vec![1.0 / n as f64; n];
    let teleport = (1.0 - alpha) / n as f64;

    for iteration in 0..max_iters {
        let per_thread_contributions: Vec<Vec<f64>> = (0..n)
            .into_par_iter()
            .fold(
                || vec![0.0; n],
                |mut local_new_rank, u| {
                    let out_degree = graph.edges[u].len();
                    if out_degree > 0 {
                        let contrib = alpha * rank[u] / out_degree as f64;
                        for &v in &graph.edges[u] {
                            local_new_rank[v] += contrib;
                        }
                    }
                    local_new_rank
                },
            )
            .collect();

        let new_rank: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|v| {
                let sum: f64 = per_thread_contributions
                    .iter()
                    .map(|buffer| buffer[v])
                    .sum();
                sum + teleport
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
