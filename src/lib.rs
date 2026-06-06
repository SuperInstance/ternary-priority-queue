//! # ternary-priority-queue
//!
//! Priority queue for GPU kernel scheduling with ternary scoring.

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct KernelJob {
    pub id: u64,
    pub name: String,
    pub ternary_score: i8,
    pub exact_priority: i32,
    pub submitted_us: u64,
}

impl PartialEq for KernelJob { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
impl Eq for KernelJob {}
impl PartialOrd for KernelJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for KernelJob {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher ternary score first, then higher exact priority, then earlier submission
        self.ternary_score.cmp(&other.ternary_score)
            .then_with(|| self.exact_priority.cmp(&other.exact_priority))
            .then_with(|| other.submitted_us.cmp(&self.submitted_us))
    }
}

pub struct TernaryPriorityQueue {
    heap: BinaryHeap<KernelJob>,
    high_count: u64,
    normal_count: u64,
    low_count: u64,
    time_us: u64,
}

impl TernaryPriorityQueue {
    pub fn new() -> Self { Self { heap: BinaryHeap::new(), high_count: 0, normal_count: 0, low_count: 0, time_us: 0 } }

    pub fn push(&mut self, name: &str, exact_priority: i32) -> u64 {
        self.time_us += 1;
        let score = if exact_priority > 50 { 1i8 } else if exact_priority < -50 { -1i8 } else { 0i8 };
        match score { 1 => self.high_count += 1, 0 => self.normal_count += 1, _ => self.low_count += 1 }
        let id = self.time_us;
        self.heap.push(KernelJob { id, name: name.into(), ternary_score: score, exact_priority, submitted_us: self.time_us });
        id
    }

    pub fn push_with_score(&mut self, name: &str, exact: i32, score: i8) -> u64 {
        self.time_us += 1;
        match score { 1 => self.high_count += 1, 0 => self.normal_count += 1, _ => self.low_count += 1 }
        let id = self.time_us;
        self.heap.push(KernelJob { id, name: name.into(), ternary_score: score, exact_priority: exact, submitted_us: self.time_us });
        id
    }

    pub fn pop(&mut self) -> Option<KernelJob> {
        let job = self.heap.pop()?;
        match job.ternary_score { 1 => self.high_count -= 1, 0 => self.normal_count -= 1, _ => self.low_count -= 1 }
        Some(job)
    }

    pub fn peek(&self) -> Option<&KernelJob> { self.heap.peek() }

    pub fn drain_high_priority(&mut self) -> Vec<KernelJob> {
        let mut high = Vec::new();
        let mut rest = Vec::new();
        while let Some(job) = self.heap.pop() {
            if job.ternary_score == 1 { high.push(job); } else { rest.push(job); }
        }
        for job in rest { self.heap.push(job); }
        self.high_count = 0;
        high
    }

    pub fn len(&self) -> usize { self.heap.len() }
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }
    pub fn high_count(&self) -> u64 { self.high_count }
    pub fn normal_count(&self) -> u64 { self.normal_count }
    pub fn low_count(&self) -> u64 { self.low_count }
}

impl Default for TernaryPriorityQueue { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut q = TernaryPriorityQueue::new();
        q.push("kernel_a", 50);
        q.push("kernel_b", 80);
        let job = q.pop().unwrap();
        assert_eq!(job.name, "kernel_b"); // higher priority
    }

    #[test]
    fn test_ternary_classify() {
        let mut q = TernaryPriorityQueue::new();
        q.push("low", -100);
        q.push("normal", 0);
        q.push("high", 100);
        assert_eq!(q.high_count(), 1);
        assert_eq!(q.normal_count(), 1);
        assert_eq!(q.low_count(), 1);
    }

    #[test]
    fn test_ordering() {
        let mut q = TernaryPriorityQueue::new();
        q.push("normal", 10);
        q.push_with_score("high", 5, 1); // manually boosted
        assert_eq!(q.pop().unwrap().name, "high"); // ternary overrides
    }

    #[test]
    fn test_drain_high() {
        let mut q = TernaryPriorityQueue::new();
        q.push("a", 100);
        q.push("b", -10);
        q.push("c", 80);
        let high = q.drain_high_priority();
        assert_eq!(high.len(), 2);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn test_peek() {
        let mut q = TernaryPriorityQueue::new();
        q.push("top", 90);
        assert_eq!(q.peek().unwrap().name, "top");
        assert_eq!(q.len(), 1); // peek doesn't remove
    }

    #[test]
    fn test_empty() {
        let mut q = TernaryPriorityQueue::new();
        assert!(q.is_empty());
        assert!(q.pop().is_none());
    }

    #[test]
    fn test_manual_score() {
        let mut q = TernaryPriorityQueue::new();
        q.push_with_score("forced_high", 10, 1);
        assert_eq!(q.peek().unwrap().ternary_score, 1);
    }
}
