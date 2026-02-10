use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::BufWriter;
use std::io::{Result, Write};

pub fn generate_random(num_nodes: usize, num_edges: usize, output_path: &str) -> Result<()> {
    let mut file = File::create(output_path)?;
    let mut rng = rand::rng();

    for _ in 0..num_edges {
        let src = rng.random_range(0..num_nodes);
        let dst = rng.random_range(0..num_nodes);
        writeln!(file, "{} {}", src, dst)?;
    }

    Ok(())
}

use rayon::prelude::*;

pub fn generate_random_parallel(
    num_nodes: usize,
    num_edges: usize,
    output_path: &str,
) -> Result<()> {
    let chunk_size = 100_000;
    let num_chunks = (num_edges + chunk_size - 1) / chunk_size;

    let chunks: Vec<String> = (0..num_chunks)
        .into_par_iter()
        .map_init(
            || rand::rng(),
            |rng: &mut rand::rngs::ThreadRng, chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = (start + chunk_size).min(num_edges);
                let mut buf = String::with_capacity((end - start) * 20);
                for _ in start..end {
                    let src = rng.random_range(0..num_nodes);
                    let dst = rng.random_range(0..num_nodes);
                    use std::fmt::Write;
                    writeln!(buf, "{} {}", src, dst).unwrap();
                }
                buf
            },
        )
        .collect();

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for chunk in &chunks {
        writer.write_all(chunk.as_bytes())?;
    }

    Ok(())
}

pub fn generate_disconnected(
    num_nodes: usize,
    num_edges: usize,
    num_components: usize,
    output_path: &str,
) -> Result<()> {
    if num_components == 0 || num_components > num_nodes {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid number of components",
        ));
    }

    let mut file = File::create(output_path)?;
    let mut rng = rand::rng();

    let nodes_per_component = num_nodes / num_components;
    let edges_per_component = num_edges / num_components;

    for comp in 0..num_components {
        let start = comp * nodes_per_component;
        let end = if comp == num_components - 1 {
            num_nodes //takes the rest
        } else {
            (comp + 1) * nodes_per_component
        };

        let edges_for_this = if comp == num_components - 1 {
            num_edges - (edges_per_component * (num_components - 1))
        } else {
            edges_per_component
        };

        for _ in 0..edges_for_this {
            let src = rng.random_range(start..end);
            let dst = rng.random_range(start..end);
            writeln!(file, "{} {}", src, dst)?;
        }
    }
    Ok(())
}

pub fn generate_line(num_nodes: usize, output_path: &str) -> Result<()> {
    let mut file = File::create(output_path)?;

    for i in 0..num_nodes - 1 {
        writeln!(file, "{} {}", i, i + 1)?;
    }

    Ok(())
}

pub fn generate_star(num_nodes: usize, output_path: &str) -> Result<()> {
    let mut file = File::create(output_path)?;

    for i in 1..num_nodes {
        writeln!(file, "0 {}", i)?;
    }

    Ok(())
}

// every node with every other, for testing dense graphs
pub fn generate_complete(num_nodes: usize, output_path: &str) -> Result<()> {
    let mut file = File::create(output_path)?;

    // let num_edges = num_nodes * (num_nodes - 1); //number of edges

    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i != j {
                writeln!(file, "{} {}", i, j)?;
            }
        }
    }

    Ok(())
}

pub fn generate_cycle(num_nodes: usize, output_path: &str) -> Result<()> {
    let mut file = File::create(output_path)?;

    for i in 0..num_nodes - 1 {
        writeln!(file, "{} {}", i, i + 1)?;
    }

    writeln!(file, "{} 0", num_nodes - 1)?;

    Ok(())
}
