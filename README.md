# Conway's Game of Life – Python & Rust

**High-Performance Computing (HPC) Project**

A project developed for the Advanced Programming Techniques course at the Faculty of Technical Sciences, University of Novi Sad.

## 1. Problem Description

Cellular automata are computational models in which space is represented as a discrete grid of cells. Each cell has a state (alive or dead), which changes in discrete time steps according to predefined rules.

This project implements **Conway's Game of Life**, a well-known cellular automaton based on four rules:

1. A live cell with fewer than 2 live neighbors **dies** (underpopulation).
2. A live cell with 2 or 3 live neighbors **survives**.
3. A live cell with more than 3 live neighbors **dies** (overpopulation).
4. A dead cell with exactly 3 live neighbors **becomes alive** (reproduction).

The project includes:

- Sequential and parallel implementations in **Python** and **Rust**.
- **Strong and weak scaling** experiments.
- Visualization of the simulation's evolution across iterations.

## 2. Implementation

### 2.1 Python Implementation

**Sequential version**
- Represents the grid as a two-dimensional matrix.
- Calculates the next generation based on neighboring cells.
- Processes the simulation iteratively.

**Parallel version (multiprocessing)**
- Divides the grid into sections assigned to worker processes.
- Computes sections concurrently.
- Combines the computed sections to produce the next generation.

### 2.2 Rust Implementation

**Sequential version**
- Represents the grid using Rust data structures.
- Computes successive generations using separate current and next states.

**Parallel version (multithreading)**
- Divides the grid into row-based segments.
- Uses `std::thread` and `Arc` for parallel computation.
- Combines computed segments after each generation.

## 3. Scaling Experiments

### 3.1 Strong Scaling

Strong scaling measures how execution time changes as the number of workers increases while the problem size remains fixed.

The experiments compare sequential and parallel implementations in both Python and Rust, evaluating execution time and speedup.

### 3.2 Weak Scaling

Weak scaling evaluates performance as the problem size increases proportionally to the number of workers.

The experiments examine scalability and parallel efficiency using different grid sizes and worker counts.

Repeated measurements were used to analyze execution times and performance variations.

## 4. Visualization

The project includes visualization of the Game of Life simulation using Rust and the **Plotters** library.

The visualization illustrates the evolution of live and dead cells across successive generations.

## 5. Architecture and Technologies

| Component | Technology |
|---|---|
| Programming Languages | Python, Rust |
| Python Parallelism | multiprocessing |
| Rust Parallelism | std::thread, Arc |
| Visualization | Plotters |
| Performance Analysis | Strong Scaling, Weak Scaling, Amdahl's Law |

### Repository Structure

```text
game-of-life-ntp/
├── python/
│   ├── src/
│   │   ├── main.py
│   │   ├── seq.py
│   │   ├── parallel.py
│   │   └── scaling.py
│   └── benchmarks/
│
├── rust/
│   ├── src/
│   │   ├── main.rs
│   │   ├── seq.rs
│   │   ├── parallel.rs
│   │   ├── scaling.rs
│   │   ├── visualize.rs
│   │   └── io.rs
│   └── Cargo.toml
│
├── pokretanje.txt
└── README.md
```

## 6. Running the Project

### Python

Navigate to the `python` directory.

**Sequential execution:**

```bash
python src/main.py --mode seq --rows 2000 --cols 2000 --iters 200
```

**Parallel execution:**

```bash
python src/main.py --mode par --rows 2000 --cols 2000 --iters 200 --workers 8
```

### Rust

Navigate to the `rust` directory.

**Sequential execution:**

```bash
cargo run --release -- --mode seq --rows 2000 --cols 2000 --iters 200
```

**Parallel execution:**

```bash
cargo run --release -- --mode par --rows 2000 --cols 2000 --iters 200 --threads 8
```

Additional execution commands and benchmarking configurations are available in [`pokretanje.txt`](pokretanje.txt).

## 7. Experimental Analysis

Detailed benchmark results, performance graphs, scaling analysis, and conclusions are available in the [Experimental Analysis Report](Izve%C5%A1taj%20o%20eksperimentima%20jakog%20i%20slabog%20skaliranja.docx).
