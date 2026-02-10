use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pdaj-projekat")]
#[command(about = "Analiza grafova za dijagnostiku kvarova", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Bfs {
        #[arg(long)]
        input: String,

        #[arg(long)]
        source: usize,

        #[arg(long)]
        mode: String,

        #[arg(long)]
        threads: Option<usize>,

        #[arg(long)]
        out: String,
    },

    Wcc {
        #[arg(long)]
        input: String,

        #[arg(long)]
        mode: String,

        #[arg(long)]
        threads: Option<usize>,

        #[arg(long)]
        out: String,
    },

    Pagerank {
        #[arg(long)]
        input: String,

        #[arg(long)]
        mode: String,

        #[arg(long)]
        threads: Option<usize>,

        #[arg(long)]
        out: String,

        #[arg(long, default_value = "0.85")]
        alpha: f64,

        #[arg(long, default_value = "50")]
        iters: usize,

        #[arg(long, default_value = "1e-10")]
        eps: f64,
    },

    Generate {
        #[arg(long)]
        graph_type: String,

        #[arg(long)]
        num_nodes: usize,

        //for random and disconnected
        #[arg(long)]
        num_edges: Option<usize>,

        //disconnected only
        #[arg(long)]
        num_components: Option<usize>,

        #[arg(long)]
        output: String,
    },
}
