//Ognjen Zeković
//E241/2025

mod bfs;
mod cli;
mod graph;
mod graph_generator;

use bfs::bfs_sequential;
use clap::Parser;
use cli::{Cli, Commands};
use graph::Graph;
use std::fs::File;
use std::io::Write;

use crate::bfs::bfs_parallel;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bfs {
            input,
            source,
            mode,
            threads,
            out,
        } => {
            println!("Loading graph from: {}", input);
            let graph = match Graph::from_file(&input) {
                Ok(g) => {
                    println!("Graph loaded: {} nodes", g.num_nodes);
                    g
                }
                Err(e) => {
                    eprintln!("Error while loading graph: {}", e);
                    std::process::exit(1);
                }
            };

            if source >= graph.num_nodes {
                eprintln!(
                    "Error: source node {} doenst exist (max is {})",
                    source,
                    graph.num_nodes - 1
                );
                std::process::exit(1);
            }

            let result = match mode.as_str() {
                "seq" => {
                    println!("Starting seq BFS from node {}...", source);
                    let start = std::time::Instant::now();
                    let res = bfs_sequential(&graph, source);
                    let duration = start.elapsed();
                    println!("BFS finished in: {:?}", duration);
                    res
                }
                "par" => {
                    println!(
                        "Starting parallel BFS from node {} with {} threads...",
                        source,
                        threads.unwrap_or(8)
                    );
                    let res;
                    let start = std::time::Instant::now();
                    match threads {
                        Some(threads) => res = bfs_parallel(&graph, source, threads),
                        None => res = bfs_parallel(&graph, source, 8),
                    }
                    let duration = start.elapsed();
                    println!("BFS finished in: {:?}", duration);
                    res
                }
                _ => {
                    eprintln!("Error: mode has to be either 'seq' or 'par'");
                    std::process::exit(1);
                }
            };

            // match save_bfs_result(&result, &out) {
            //     Ok(_) => println!("Result saved in: {}", out),
            //     Err(e) => {
            //         eprintln!("Error while saving results: {}", e);
            //         std::process::exit(1);
            //     }
            // }

            print_bfs_stats(&result, source);
        }

        Commands::Wcc { .. } => {
            println!("WCC not implemented");
            std::process::exit(1);
        }

        Commands::Pagerank { .. } => {
            println!("Pagerank not implemented");
            std::process::exit(1);
        }

        Commands::Generate {
            graph_type,
            num_nodes,
            num_edges,
            output,
        } => {
            use graph_generator::*;

            let result = match graph_type.as_str() {
                "line" => generate_line(num_nodes, &output),
                "star" => generate_star(num_nodes, &output),
                "complete" => generate_complete(num_nodes, &output),
                "cycle" => generate_cycle(num_nodes, &output),
                "random" => match num_edges {
                    Some(edges) => generate_random(num_nodes, edges, &output),
                    None => {
                        eprintln!("Error: random graph demands number of edges");
                        eprintln!("   Example: cargo run -- generate random 100 500 output.txt");
                        std::process::exit(1);
                    }
                },
                _ => {
                    eprintln!("Error: bad type of graph '{}'", graph_type);
                    eprintln!("   Available graphs: line, star, complete, cycle, random");
                    std::process::exit(1);
                }
            };

            match result {
                Ok(_) => {
                    println!("Successfully generated graph");
                }
                Err(e) => {
                    eprintln!("Error while generating graph: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn save_bfs_result(result: &[i32], path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    for (node, dist) in result.iter().enumerate() {
        writeln!(file, "{} {}", node, dist)?;
    }

    Ok(())
}

fn print_bfs_stats(result: &[i32], source: usize) {
    let reachable = result.iter().filter(|&&d| d != -1).count();
    let unreachable = result.iter().filter(|&&d| d == -1).count();
    let max_dist = result.iter().filter(|&&d| d != -1).max().unwrap_or(&0);

    println!("\nStatistics:");
    println!("   Source node: {}", source);
    println!("   Reachable nodes: {}", reachable);
    println!("   Unreachable nodes: {}", unreachable);
    println!("   Max dist: {}", max_dist);
}
