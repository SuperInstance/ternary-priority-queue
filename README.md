# ternary-priority-queue

Priority queue for GPU kernel scheduling with ternary scoring. Items scored {-1=deprioritize, 0=normal, +1=prioritize}. O(1) ternary classify, O(log n) exact ordering.

## Overview

# ternary-priority-queue

Priority queue for GPU kernel scheduling with ternary scoring.

## Stats

- **Tests**: 7
- **LOC**: 151
- **License**: Apache-2.0

## Part of the Oxide Stack

This crate is part of the [Flux→PTX](https://github.com/SuperInstance/cuda-oxide/blob/main/FLUX_TO_PTX.md) experimental suite, testing synergies between the five layers of the distributed GPU runtime:

1. **open-parallel** — async runtime (tokio fork)
2. **pincher** — "Vector DB as runtime, LLM as compiler"
3. **flux-core** — bytecode VM + A2A agent protocol
4. **cuda-oxide** — Flux→MIR→Pliron→NVVM→PTX compiler
5. **cudaclaw** — persistent GPU kernels, warp-level consensus, SmartCRDT

## Usage

```rust
use ternary_priority_queue::*;
// See tests in src/lib.rs for examples
```

## License

Apache-2.0
