import subprocess
import re
import time
import pandas as pd
import numpy as np
from pathlib import Path

ALGORITHMS = ['bfs', 'wcc', 'pagerank']
GRAPHS = [
    {'path': 'test_graphs/random_m.txt', 'size': 100000, 'name': 'small'},
    {'path': 'test_graphs/random_l.txt', 'size': 1000000, 'name': 'large'},
]
THREADS = [1, 2, 4, 8]
ITERATIONS = 3

def parse_time(output: str) -> float:
    match = re.search(r'finished in: ([\d.]+)(ms|µs|s)', output)
    if not match:
        return None
    
    value = float(match.group(1))
    unit = match.group(2)
    
    # Konvertuj u ms
    if unit == 's':
        return value * 1000
    elif unit == 'ms':
        return value
    elif unit == 'µs':
        return value / 1000
    
    return None

def run_command(algo: str, graph_path: str, mode: str, threads: int) -> tuple:
    """Pokreni benchmark komandu"""
    cmd = [
        'cargo', 'run', '--release', '--',
        algo,
        '--input', graph_path,
        '--mode', mode,
        '--out', '/dev/null'
    ]
    
    if algo == 'bfs':
        cmd.extend(['--source', '0'])
    
    if mode == 'par':
        cmd.extend(['--threads', str(threads)])
    
    if algo == 'pagerank':
        cmd.extend(['--alpha', '0.85', '--iters', '50', '--eps', '1e-10'])
    
    start = time.time()
    result = subprocess.run(cmd, capture_output=True, text=True)
    elapsed = (time.time() - start) * 1000  # ms
    
    parsed_time = parse_time(result.stdout + result.stderr)
    
    return parsed_time if parsed_time else elapsed, result.returncode == 0


def main():
    print("Starting benchmark...")
    print(f"   Algorithms: {ALGORITHMS}")
    print(f"   Thread configs: {THREADS}")
    print(f"   Iterations: {ITERATIONS}")
    print()
    
    results = []
    
    for graph in GRAPHS:
        graph_path = Path(graph['path'])
        
        if not graph_path.exists():
            print(f"Graph not found: {graph_path} (skipping)")
            continue
        
        print(f"\nBenchmarking: {graph['name']} (size: {graph['size']})")
        print("=" * 60)
        
        for algo in ALGORITHMS:
            print(f"\n  Algorithm: {algo}")
            print("  " + "-" * 50)
            
            # Sequential
            for iteration in range(1, ITERATIONS + 1):
                print(f"    seq, iter {iteration}...", end=' ', flush=True)
                time_ms, success = run_command(algo, str(graph_path), 'seq', 1)
                
                if success:
                    print(f"{time_ms:.2f}ms")
                    results.append({
                        'Algorithm': algo,
                        'Graph': graph['name'],
                        'Size': graph['size'],
                        'Mode': 'seq',
                        'Threads': 1,
                        'Iteration': iteration,
                        'Time_ms': time_ms
                    })
                else:
                    print("Failed")
            
            # Parallel
            for threads in THREADS:
                if threads == 1:
                    continue
                
                for iteration in range(1, ITERATIONS + 1):
                    print(f"    par (t={threads}), iter {iteration}...", end=' ', flush=True)
                    time_ms, success = run_command(algo, str(graph_path), 'par', threads)
                    
                    if success:
                        print(f"{time_ms:.2f}ms")
                        results.append({
                            'Algorithm': algo,
                            'Graph': graph['name'],
                            'Size': graph['size'],
                            'Mode': 'par',
                            'Threads': threads,
                            'Iteration': iteration,
                            'Time_ms': time_ms
                        })
                    else:
                        print("Failed")
    
    df = pd.DataFrame(results)
    df.to_csv('benchmark_results.csv', index=False)
    print("\n✓ Raw results saved to: benchmark_results.csv")
    
    summary = df.groupby(['Algorithm', 'Graph', 'Size', 'Mode', 'Threads'])['Time_ms'].agg([
        ('Mean_ms', 'mean'),
        ('Std_ms', 'std'),
        ('Min_ms', 'min'),
        ('Max_ms', 'max')
    ]).reset_index()

    def calc_speedup(row):
        algo = row['Algorithm']
        graph = row['Graph']
        
        seq_time = summary[
            (summary['Algorithm'] == algo) & 
            (summary['Graph'] == graph) & 
            (summary['Mode'] == 'seq')
        ]['Mean_ms']
        
        if len(seq_time) > 0:
            return seq_time.iloc[0] / row['Mean_ms']
        return 1.0
    
    summary['Speedup'] = summary.apply(calc_speedup, axis=1)
    summary['Efficiency'] = summary['Speedup'] / summary['Threads']
    
    summary.to_csv('benchmark_summary.csv', index=False)
    print("Summary saved to: benchmark_summary.csv")
    
    print("\n" + "=" * 100)
    print("BENCHMARK SUMMARY")
    print("=" * 100)
    
    pd.set_option('display.max_rows', None)
    pd.set_option('display.width', 120)
    print(summary.to_string(index=False))
    
    print("\n" + "=" * 100)
    print("TOP SPEEDUPS")
    print("=" * 100)
    
    best_speedups = summary[summary['Threads'] > 1].nlargest(10, 'Speedup')[
        ['Algorithm', 'Graph', 'Threads', 'Mean_ms', 'Speedup']
    ]
    print(best_speedups.to_string(index=False))
    
    print("\nBenchmark complete!")

if __name__ == '__main__':
    main()