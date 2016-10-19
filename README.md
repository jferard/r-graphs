# R-Graphs
A simple graph library for Rust.

Copyright (C) 2016 J. Férard <https://github.com/jferard>

## Why?
Just learning Rust.

## Features
* Simple graphs, with no decoration (no weigthed, colored,... nodes or edges);
* Two simple algorithms: DFS (Depth First Search) and BFS (Breadth First Search);
* A [Graphviz](www.graphviz.org) output, step by step.

## Compilation
*Requires rustc 1.14.0-nightly.*
First step: clone the repository with `git clone https://github.com/jferard/r-graphs`
Then type: `cargo build` or `cargo test`. The latter will create some graphs in the `gv_output` directory.

## TODO
A lot.