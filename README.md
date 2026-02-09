# graph-analysis
Project covers analysis of graph algorithms BFS, WCC and PageRank in Rust programming language, in education purposes of subject Parallel and distributed architectures and languages, FTN Novi Sad

# RUN
cargo build --release

## Show commands
cargo run -- --help

## BFS
- cargo run -- bfs --input test_graphs\random_small.txt --source 0 --mode seq --out bfs.txt
- cargo run --release -- bfs --input test_graphs\random_small.medium --source 0 --mode par --out bfs.txt

## WCC
- cargo run --release -- wcc --input test_graphs\random_small.txt --mode seq --out wcc.txt
- cargo run --release -- wcc --input test_graphs\random_small.txt --mode par --threads 8 --out wcc.txt

## PAGERANK
- cargo run --release -- pagerank --input test_graphs\random_small.txt --mode seq --alpha 0.85 --out pagerank.txt
- cargo run --release -- pagerank --input test_graphs\random_small.txt --mode par --threads 8 --out pagerank.txt --alpha 0.85 --iters 50 --eps 1e-10

# Test graphs creation
mkdir -p test_graphs
## Small
- cargo run --release -- generate --graph-type line --num-nodes 100 --output test_graphs/line_small.txt
- cargo run --release -- generate --graph-type star --num-nodes 100 --output test_graphs/star_small.txt
- cargo run --release -- generate --graph-type cycle --num-nodes 100 --output test_graphs/cycle_small.txt
- cargo run --release -- generate --graph-type random --num-nodes 100 --num-edges 500 --output test_graphs/random_small.txt

## Medium
- cargo run --release -- generate --graph-type line --num-nodes 10000 --output test_graphs/line_medium.txt
- cargo run --release -- generate --graph-type random --num-nodes 10000 --num-edges 50000 --output test_graphs/random_medium.txt

## Large
- cargo run --release -- generate --graph-type random --num-nodes 100000 --num-edges 1000000 --output test_graphs/random_large.txt
