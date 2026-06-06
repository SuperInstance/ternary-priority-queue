# ternary-priority-queue

Priority queue for GPU kernel scheduling with ternary scoring. Items scored {-1=deprioritize, 0=normal, +1=prioritize}. O(1) ternary classify, O(log n) exact ordering.

## Why This Matters

# ternary-priority-queue
Priority queue for GPU kernel scheduling with ternary scoring.

## The Five-Layer Stack

This crate is part of the **Oxide Stack** — a distributed GPU runtime built on five layers:

```
┌─────────────────┐
│  cudaclaw        │  Persistent GPU kernels, warp consensus, SmartCRDT
├─────────────────┤
│  cuda-oxide      │  Flux → MIR → Pliron → NVVM → PTX compiler
├─────────────────┤
│  flux-core       │  Bytecode VM + A2A agent protocol
├─────────────────┤
│  pincher         │  "Vector DB as runtime, LLM as compiler"
├─────────────────┤
│  open-parallel   │  Async runtime (tokio fork)
└─────────────────┘
```

The key insight: **ternary values {-1, 0, +1} map directly to GPU compute**. They pack 16× denser than FP32, enable XNOR+popcount matmul, and conservation laws become compile-time checks.

## Design

Every value in this crate follows **ternary algebra** (Z₃):

| Value | Meaning | GPU Analog |
|-------|---------|------------|
| +1 | Positive / Active / Healthy | Warp vote yes |
| 0 | Neutral / Pending / Balanced | Warp vote abstain |
| -1 | Negative / Failed / Overloaded | Warp vote no |

This isn't arbitrary — ternary is the natural encoding for:
1. **BitNet b1.58** (Microsoft) — ternary LLMs at 60% less power
2. **GPU warp voting** — hardware ballot returns ternary consensus
3. **Conservation laws** — {-1, 0, +1} preserves quantity

## Key Types

```rust
pub struct KernelJob
pub struct TernaryPriorityQueue
pub fn new
pub fn push
pub fn push_with_score
pub fn pop
pub fn peek
pub fn drain_high_priority
pub fn len
pub fn is_empty
pub fn high_count
pub fn normal_count
```

## Usage

```toml
[dependencies]
ternary-priority-queue = "0.1.0"
```

```rust
use ternary_priority_queue::*;
// See src/lib.rs tests for complete working examples
```

## Testing

```bash
git clone https://github.com/SuperInstance/ternary-priority-queue.git
cd ternary-priority-queue
cargo test    # 7 tests
```

## Stats

| Metric | Value |
|--------|-------|
| Tests | 7 |
| Lines of Rust | 152 |
| Public API | 13 items |

## License

Apache-2.0
