# flipr
Functional Library for Image Processing in Rust

A composable, iterator-like API for image processing with support for multiple backends and transformations.

## Overview

Flipr provides a functional approach to image processing, inspired by Rust's `std::iter::Iterator`. It allows you to build image processing pipelines that can be executed on different backends (CPU, GPU) while maintaining clean, composable code.

## Architecture

The library is organized into multiple workspaces:

### `flipr-core`
Core traits and types for image processing. Provides the `ImageProcessor` trait, which is the foundation for building processing pipelines.

**Key features:**
- Iterator-like trait for functional composition
- `map()`, `filter()`, and `chain()` methods for pipeline building
- Basic pixel types (`Rgb`, `Gray`)

### `flipr-transform`
Affine space transformations for images.

**Key features:**
- Affine transformation matrices
- Translation, rotation, and scaling operations
- Transform composition and inversion
- Extension trait for easy use with image processors

### `flipr-ops`
Operation descriptions with different execution backends.

**Key features:**
- Abstract operation descriptions
- CPU backend implementation
- GPU backend interface (placeholder)
- Operation builder pattern

### `flipr-macros`
Procedural macros for transforming Rust functions into operation descriptions.

**Key features:**
- `#[image_op]` - Convert functions to operations
- `#[gpu_compatible]` - Mark functions for GPU execution
- `#[derive(Operation)]` - Generate operation implementations

## Usage Examples

### Basic Image Processing

```rust
use flipr_core::{ImageProcessor, Gray};

// Create a processor and apply transformations
let processor = MyImageProcessor::new();
let processed = processor
    .map(|pixel: Gray<u8>| Gray { value: pixel.value * 2 })
    .filter(|pixel| pixel.value > 128);

// Process individual pixels
if let Ok(Some(pixel)) = processed.process_pixel(10, 20) {
    println!("Pixel value: {}", pixel.value);
}
```

### Affine Transformations

```rust
use flipr_transform::{AffineTransform, TransformExt};

let processor = MyImageProcessor::new();

// Apply rotation
let rotated = processor.rotate(std::f64::consts::PI / 4.0);

// Compose transformations
let transform = AffineTransform::translation(10.0, 20.0)
    .then(&AffineTransform::scale(2.0, 2.0));
let transformed = processor.transform(transform);
```

### Backend Execution

```rust
use flipr_ops::{CpuBackend, GpuBackend, Operation, OperationBuilder, PointwiseOp};

// Create an operation
let op = OperationBuilder::pointwise(PointwiseOp::Brighten(1.5));

// Execute on CPU
let cpu_backend = CpuBackend;
let result = cpu_backend.execute(&op);

// Or try GPU execution
let gpu_backend = GpuBackend::new(0);
let result = gpu_backend.execute(&op);
```

### Procedural Macros

```rust
use flipr_macros::image_op;

#[image_op]
fn brighten(pixel: u8, amount: f64) -> u8 {
    (pixel as f64 * amount).min(255.0) as u8
}

// Use the generated operation builder
let op_name = brighten::name();
```

## Building

Build the entire workspace:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Build individual crates:

```bash
cargo build -p flipr-core
cargo build -p flipr-transform
cargo build -p flipr-ops
cargo build -p flipr-macros
```

## Design Principles

1. **Composability**: Like Rust's iterators, image processors can be chained and composed
2. **Type Safety**: Strong typing ensures correctness at compile time
3. **Backend Agnostic**: Operations can be described once and executed on different backends
4. **Zero-Cost Abstractions**: High-level API without runtime overhead

## License

Licensed under the Apache License, Version 2.0. See LICENSE file for details.

## Contributing

Contributions are welcome! This library is in early development and there are many opportunities to contribute:

- Implement additional transformations
- Add GPU backend support
- Optimize existing operations
- Improve documentation
- Add examples

