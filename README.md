# ThrillerFlow

## Introduction
**ThrillerFlow** is a **Dataflow Analysis** and **Codegen** Framework written in Rust. 

In **ThrillerFlow**, we introduce a nested multi-dimendional dataflow
graph called **Extended task Dependence Graph(ETDG)**, a unified intermediate
representation that preserves a holistic view of parallelism and
dependency across different control and data nested levels on the code.
To facilitate code analysis and the later low-level code generation,
an ETDG concisely encodes complex control structures and precisely
represents the iteration-level data dependencies with and acyclic graph.
For a clear exposition, ETDG borrows the concepts from the reduced dependence
graph used classical compilers with the **Static Control Program(SCoP)** modeling
employed in polyhedral compilers.

## Quick Start

### Download
```
git clone git@github.com:TiledTensor/ThrillerFlow.git
```

### Unit Test
```
make test
```

### Example
```
make example
```

## Usage

Add the following lines to your Cargo.toml:

```toml
[dependencies]
thriller_flow = { git = "https://github.com/TiledTensor/ThrillerFlow.git" }
```

## License
MIT License
