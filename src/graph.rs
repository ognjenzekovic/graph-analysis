use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph {
    pub num_nodes: usize,
    pub edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut edges_temp: Vec<(usize, usize)> = Vec::new();
        let mut max_node = 0;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() || line.starts_with("//") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let src: usize = parts[0].parse()?;
                let dst: usize = parts[1].parse()?;
                edges_temp.push((src, dst));
                max_node = max_node.max(src).max(dst);
            }
        }

        let num_nodes = max_node + 1;
        let mut edges = vec![Vec::new(); num_nodes];

        for (src, dst) in edges_temp {
            edges[src].push(dst);
        }

        Ok(Graph { num_nodes, edges })
    }
}
