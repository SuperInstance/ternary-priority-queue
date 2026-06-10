# ternary-priority-queue

Priority queue for GPU kernel scheduling with ternary scoring — high (+1), normal (0), low (-1).

## Why This Exists

GPU kernels have different urgency levels. A real-time inference kernel is high priority. A background batch job is low priority. A maintenance kernel (defragmentation, checkpoint) is normal. This crate wraps a binary heap with ternary classification: every job gets an exact priority number for ordering, plus a ternary score (-1, 0, +1) for fast categorization. You can drain all high-priority kernels first, check the distribution of priorities, and schedule accordingly.

## Architecture

### Core Types

- **`KernelJob`** — A scheduled job with `id`, `name`, `exact_priority` (fine-grained), `ternary_score` (-1/0/+1), and `enqueue_time_us`.
- **`TernaryPriorityQueue`** — Binary heap wrapper tracking counts per priority tier.

### Priority Logic

- `push(name, exact)`: Auto-assigns ternary score based on exact priority thresholds.
- `push_with_score(name, exact, score)`: Manual ternary score override.
- `drain_high_priority`: Extract all +1 jobs for immediate dispatch.

## Usage

```rust
use ternary_priority_queue::TernaryPriorityQueue;

let mut pq = TernaryPriorityQueue::new();

pq.push("realtime_inference", 100);  // high priority
pq.push("batch_training", 10);       // low priority
pq.push("checkpoint", 50);           // normal
pq.push_with_score("emergency", 200, 1); // force high

// Drain all critical work first
let critical = pq.drain_high_priority();
assert_eq!(critical.len(), 2);

// Pop next highest
let next = pq.pop();
```

## API Reference

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `TernaryPriorityQueue` | Create empty queue |
| `push(name, exact_priority)` | `u64` | Enqueue with auto ternary score |
| `push_with_score(name, exact, score)` | `u64` | Enqueue with manual ternary score |
| `pop()` | `Option<KernelJob>` | Dequeue highest priority |
| `peek()` | `Option<&KernelJob>` | Look at top without removing |
| `drain_high_priority()` | `Vec<KernelJob>` | Extract all +1 jobs |
| `len()` / `is_empty()` | `usize` / `bool` | Queue size |
| `high_count()` / `normal_count()` / `low_count()` | `u64` | Per-tier counts |

## The Deeper Idea

Ternary priority is a **two-level scheduling signal**. The exact priority gives fine-grained ordering within a tier. The ternary score gives coarse-grained routing: high-priority jobs go to the fast lane (dedicated GPU stream), normal go to the shared stream, and low-priority jobs get whatever's left. This avoids the complexity of a full priority scheduler while giving you the most important scheduling decision (fast lane vs. slow lane) essentially for free.

## Related Crates

- **ternary-semaphore** — resource permits with ternary capacity
- **ternary-dispatch** — kernel dispatch with ternary-packed payloads
- **ternary-backpressure** — backpressure for pipeline stages
