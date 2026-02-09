# graph-analysis
Project covers analysis of graph algorithms BFS, WCC and PageRank in Rust programming language, in education purposes of subject Parallel and distributed architectures and languages, FTN Novi Sad

# RUN
cargo build --release

## Show commands
cargo run -- --help

## BFS
- cargo run --release -- bfs --input test_graphs\random_l.txt --source 0 --mode seq --out bfs.txt
- cargo run --release -- bfs --input test_graphs\random_l.txt --source 0 --mode par --out bfs.txt

## WCC
- cargo run --release -- wcc --input test_graphs\random_l.txt --mode seq --out wcc.txt
- cargo run --release -- wcc --input test_graphs\random_l.txt --mode par --threads 8 --out wcc.txt

## PAGERANK
- cargo run --release -- pagerank --input test_graphs\random_l.txt --mode seq --alpha 0.85 --out pagerank.txt
- cargo run --release -- pagerank --input test_graphs\random_l.txt --mode par --threads 8 --out pagerank.txt --alpha 0.85 --iters 50 --eps 1e-10

# Test graphs creation
mkdir -p test_graphs
## Small (testing)
- cargo run --release -- generate --graph-type line --num-nodes 10000 --output test_graphs/line_s.txt
- cargo run --release -- generate --graph-type star --num-nodes 10000 --output test_graphs/star_s.txt
- cargo run --release -- generate --graph-type cycle --num-nodes 10000 --output test_graphs/cycle_s.txt
- cargo run --release -- generate --graph-type random --num-nodes 10000 --num-edges 50000 --output test_graphs/random_s.txt
- (wcc*) cargo run --release -- generate --graph-type disconnected --num-nodes 1000 --num-edges 5000 --num-components 5 --output test_graphs\disconnected.txt

## Medium
- cargo run --release -- generate --graph-type random --num-nodes 100000 --num-edges 1000000 --output test_graphs/random_m.txt

## Large
- cargo run --release -- generate --graph-type random --num-nodes 1000000 --num-edges 10000000 --output test_graphs/random_l.txt
