[![Build Status](https://travis-ci.org/jferard/fastods.svg?branch=master)](https://travis-ci.org/jferard/r-graphs)
[![Code Coverage](https://img.shields.io/codecov/c/github/jferard/fastods/master.svg)](https://codecov.io/github/jferard/r-graphs?branch=master)

# R-Graphs
A simple graph library for Rust.

Copyright (C) 2016 J. Férard <https://github.com/jferard>

## Why?
Just learning Rust.

## Features
* Simple graphs, with no decoration (no weigthed, colored,... nodes or edges);
* Two simple algorithms: DFS (Depth First Search) and BFS (Breadth First Search);
* A [Graphviz](http://www.graphviz.org) output, step by step.

## Compilation
*Requires rustc 1.14.0-nightly.*
First step: clone the repository with `git clone https://github.com/jferard/r-graphs`
Then type: `cargo build` or `cargo test`. The latter will create some graphs in the `gv_output` directory.

## View the output
On Linux:
```dot [file].dot -Tsvg | display```

## TODO
A lot.