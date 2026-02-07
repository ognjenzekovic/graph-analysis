use rand::Rng;
use std::fs::File;
use std::io::{Result, Write};

pub fn generate_random(num_nodes: usize, num_edges: usize, output_path: &str) -> Result<()> {
    let mut file = File::create(output_path)?;
    let mut rng = rand::thread_rng();

    for _ in 0..num_edges {
        let src = rng.gen_range(0..num_nodes);
        let dst = rng.gen_range(0..num_nodes);
        writeln!(file, "{} {}", src, dst)?;
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
