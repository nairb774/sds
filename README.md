# sds: A Streaming Differential Set Library

`sds` is an experimental Rust library for incremental stream processing. The core concept is to build a streaming data processing system where the base primitive is a **differentiable set**.

## Core Concepts

- **Streams of Updates:** Data is represented as a continuous stream of updates to a multi-set. These updates are captured in an `Update<T>` struct, which contains an `item` of type `T` and a `diff` (a non-zero integer) representing how the count of that item has changed. For example, `{ item: A, diff: 1 }` represents an addition, while `{ item: A, diff: -1 }` represents a removal.
- **Sets as State:** The underlying state of any stream is a sorted set of elements. This state can be stored in various backends.
- **Pluggable Storage:** The storage layer is designed to be pluggable via the `Set<T>` trait. The initial implementation uses an in-memory `BTreeSet`, but the design allows for other backends (like RocksDB) in the future.
- **Composable Operators:** The library provides a set of operators that can be chained together to build complex data processing pipelines. These operators work on streams of updates to incrementally update their output.

### Available Operators

The following stateless operators and functions are currently available:

- **`Map<In, Out, F>`:** Applies a function to each item in the stream, transforming it from type `In` to `Out`.
- **`Filter<T, F>`:** Filters the stream, allowing only items that satisfy a given predicate to pass through.
- **`FlatMap<In, Out, F, I>`:** A more powerful version of `map` and `filter`. It applies a function to each item that returns an iterator, and the results are flattened into a single output stream. This can be used to create, discard, or multiply items.
- **`compact<T, I>`:** A function that takes a stream of updates and produces a minimal, logically equivalent stream. It does this by summing the `diff`s for each item and filtering out any items whose total `diff` is zero. This is useful for reducing the volume of data in a stream and optimizing downstream processing.

## Project Goals

The primary goal of this project is to explore the viability of building a flexible and performant streaming data processing library based on these simple set-based primitives. We aim to create a system that is:

- **Efficient:** By processing only the changes (deltas), we can avoid re-computing results over the entire dataset.
- **Flexible:** The pluggable storage and composable operator model should allow for a wide range of use cases.
- **Ergonomic:** The library should provide a simple and intuitive API for developers to build their streaming applications.

This repository currently contains the initial scaffolding for the library, including the core traits, an in-memory implementation, and a CI workflow to ensure code quality.