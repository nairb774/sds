# sds: A Streaming Differential Set Library

`sds` is an experimental Rust library for incremental stream processing. The core concept is to build a streaming data processing system where the base primitive is a **differentiable set**.

## Core Concepts

- **Streams of Updates:** Data is represented as a continuous stream of updates to a set. These updates are simple `Add(T)` and `Remove(T)` operations on individual elements.
- **Sets as State:** The underlying state of any stream is a sorted set of elements. This state can be stored in various backends.
- **Pluggable Storage:** The storage layer is designed to be pluggable via the `Set<T>` trait. The initial implementation uses an in-memory `BTreeSet`, but the design allows for other backends (like RocksDB) in the future.
- **Composable Operators:** The library will provide a set of operators (like `map`, `filter`, `group_by`, etc.) that can be chained together to build complex data processing pipelines. These operators work on the streams of updates to incrementally update their output.

## Project Goals

The primary goal of this project is to explore the viability of building a flexible and performant streaming data processing library based on these simple set-based primitives. We aim to create a system that is:

- **Efficient:** By processing only the changes (deltas), we can avoid re-computing results over the entire dataset.
- **Flexible:** The pluggable storage and composable operator model should allow for a wide range of use cases.
- **Ergonomic:** The library should provide a simple and intuitive API for developers to build their streaming applications.

This repository currently contains the initial scaffolding for the library, including the core traits, an in-memory implementation, and a CI workflow to ensure code quality.