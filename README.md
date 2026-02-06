# graph-analysis
Project covers analysis of graph algorithms BFS, WCC and PageRank in Rust programming language, in education purposes of subject Parallel and distributed architectures and languages, FTN Novi Sad

# RUN
cargo build --release

## Show commands
cargo run -- --help

## Run BFS
cargo run -- bfs --input test_graph.txt --source 0 --mode seq --out bfs_output.txt

# Test graphs creation
## Small
cargo run --release -- generate line 100 test_graphs/line_small.txt
cargo run --release -- generate star 100 test_graphs/star_small.txt
cargo run --release -- generate random 100 500 test_graphs/random_small.txt

## Medium
cargo run --release -- generate line 10000 test_graphs/line_medium.txt
cargo run --release -- generate random 10000 50000 test_graphs/random_medium.txt

## Big
cargo run --release -- generate random 100000 1000000 test_graphs/random_large.txt
