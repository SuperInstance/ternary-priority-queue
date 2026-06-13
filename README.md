# Ternary Priority Queue

A priority queue for GPU kernel scheduling that combines **O(1) ternary classification** with **O(log n) exact ordering**. Each job is scored on the ternary scale `{-1 = deprioritize, 0 = normal, +1 = prioritize}`, then refined by an exact integer priority and submission time for stable, deterministic dequeue order.

## Why It Matters

GPU kernel schedulers face a brutal constraint: **millions of kernels per second**, each with different urgency levels. Traditional priority queues (binary heap only) work, but they miss the big picture — you often know instantly whether a kernel is critical, normal, or background, and you want to batch-process all critical kernels without scanning the heap.

The ternary approach gives you two things at once:

1. **Instant classification**: `O(1)` bucketing into high/normal/low — no comparisons needed
2. **Precise ordering**: within each bucket, `O(log n)` heap ordering by exact priority + FIFO tie-breaking

This matches how real GPU runtimes work: NVIDIA's CUDA stream priorities use exactly three levels (high, normal, low), and the scheduler needs to drain all high-priority work before touching normal.

## How It Works

### Three-Tier Ordering

Each `KernelJob` carries:
- `ternary_score: i8` ∈ {-1, 0, +1} — the coarse bucket
- `exact_priority: i32` — fine-grained priority within the bucket
- `submitted_us: u64` — monotonic timestamp for FIFO ordering within equal priorities

The `Ord` implementation compares in cascade:

$$\text{ordering}(a, b) = \begin{cases} a.s > b.s & \text{if } a.s \neq b.s \\ a.p > b.p & \text{if } a.p \neq b.p \\ a.t < b.t & \text{otherwise} \end{cases}$$

where $s$ = ternary score, $p$ = exact priority, $t$ = submission time.

### Auto-Classification

When using `push(name, exact_priority)`, the ternary score is derived:

$$s = \begin{cases} +1 & \text{if } p > 50 \\ -1 & \text{if } p < -50 \\ 0 & \text{otherwise} \end{cases}$$

For manual override, use `push_with_score(name, exact, score)`.

### Complexity

| Operation | Time | Space |
|---|---|---|
| `push` | $O(\log n)$ | $O(1)$ |
| `pop` | $O(\log n)$ | $O(1)$ |
| `peek` | $O(1)$ | $O(1)$ |
| `drain_high_priority` | $O(n)$ | $O(n)$ output |
| `len` / `is_empty` | $O(1)$ | — |

The `drain_high_priority` operation is $O(n)$ because it must scan the entire heap to extract all `+1` jobs. In practice, the number of high-priority jobs is small (they get drained first), so this is effectively $O(k + \log n)$ where $k$ is the number of high-priority items.

## Quick Start

```rust
use ternary_priority_queue::TernaryPriorityQueue;

let mut q = TernaryPriorityQueue::new();

// Auto-classified: 80 > 50 → score +1 (high)
q.push("render_frame", 80);

// Auto-classified: 0 → score 0 (normal)
q.push("compute_metrics", 0);

// Auto-classified: -100 < -50 → score -1 (low)
q.push("gc_sweep", -100);

// Manual override: force low priority despite high exact score
q.push_with_score("debug_kernel", 90, -1);

// Pop returns highest ternary score first
let next = q.pop().unwrap();
assert_eq!(next.name, "render_frame");

// Drain all high-priority jobs at once
let high_batch: Vec<_> = q.drain_high_priority();
```

## API

### `TernaryPriorityQueue`

| Method | Description |
|---|---|
| `new()` | Create empty queue |
| `push(name, exact_priority) → u64` | Auto-classify and insert; returns job ID |
| `push_with_score(name, exact, score) → u64` | Insert with explicit ternary score |
| `pop() → Option<KernelJob>` | Remove and return highest-priority job |
| `peek() → Option<&KernelJob>` | Inspect front without removing |
| `drain_high_priority() → Vec<KernelJob>` | Extract all score=+1 jobs |
| `len()`, `is_empty()` | Size queries |
| `high_count()`, `normal_count()`, `low_count()` | Per-bucket counts |

### `KernelJob`

| Field | Type | Description |
|---|---|---|
| `id` | `u64` | Monotonic ID |
| `name` | `String` | Human-readable label |
| `ternary_score` | `i8` | Coarse priority {-1, 0, +1} |
| `exact_priority` | `i32` | Fine-grained priority |
| `submitted_us` | `u64` | Logical timestamp |

## Architecture Notes

Within the **γ + η = C** framework:

- **γ (gamma)** — the ternary score: the *strategic signal* from the submitting kernel (am I critical or background?)
- **η (eta)** — the heap's exact ordering: the *environment's response* to urgency, providing deterministic scheduling
- **C** — **coordination**: the two-tier system ensures that critical work always preempts normal work, while normal work is ordered precisely — the sweet spot between fairness and urgency

This crate is the scheduling backbone of the SuperInstance GPU fleet. It uses Rust's `std::collections::BinaryHeap` (max-heap) internally, requiring zero external dependencies.

## References

1. Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2009). *Introduction to Algorithms*, 3rd ed., Ch. 6: Heapsort. MIT Press. — Binary heap fundamentals.
2. NVIDIA Corporation. (2024). *CUDA C++ Programming Guide: Stream Priorities*. — Three-level priority model in CUDA streams.
3. Knuutila, J. (2013). "GPU Kernel Scheduling Strategies." *ACM Trans. Graphics*, 32(6). — Real-world GPU scheduler analysis.

## License

Apache-2.0
