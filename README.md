# graph-analysis
Project covers analysis of graph algorithms BFS, WCC and PageRank in Rust programming language, in education purposes of subject Parallel and distributed architectures and languages, FTN Novi Sad

# RUN
cargo build --release

## Show commands
cargo run -- --help

## Run BFS
cargo run -- bfs --input test_graph.txt --source 0 --mode seq --out bfs_output.txt

# Test graphs creation
mkdir -p test_graphs
## Small
cargo run --release -- generate --graph-type line --num-nodes 100 --output test_graphs/line_small.txt
cargo run --release -- generate --graph-type star --num-nodes 100 --output test_graphs/star_small.txt
cargo run --release -- generate --graph-type cycle --num-nodes 100 --output test_graphs/cycle_small.txt
cargo run --release -- generate --graph-type random --num-nodes 100 --num-edges 500 --output test_graphs/random_small.txt

## Medium
cargo run --release -- generate --graph-type line --num-nodes 10000 --output test_graphs/line_medium.txt
cargo run --release -- generate --graph-type random --num-nodes 10000 --num-edges 50000 --output test_graphs/random_medium.txt

## Large
cargo run --release -- generate --graph-type random --num-nodes 100000 --num-edges 1000000 --output test_graphs/random_large.txt
