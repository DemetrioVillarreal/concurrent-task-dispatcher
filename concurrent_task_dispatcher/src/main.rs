use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
enum TaskKind {
    IO,
    CPU,
}

#[derive(Clone)]
struct Task {
    id: usize,
    arrival_time: u128,
    kind: TaskKind,
    duration_ms: u64,
    cpu_cost: u32,
    created_at: Instant,
}

struct CompletedTask {
    id: usize,
    kind: TaskKind,
    wait_ms: u128,
    turnaround_ms: u128,
    cpu_cost: u32,
    worker_id: usize,
}

#[derive(Clone, Copy)]
enum Policy {
    Fifo,
    Optimized,
}

#[derive(Clone)]
struct Config {
    total_tasks: usize,
    workers: usize,
    io_percent: u32,
    interval_ms: u64,
    duration_ms: u64,
    seed: u64,
}

struct SharedState {
    current_cpu: u32,
    active_workers: usize,
    done: bool,
}

struct MonitorResult {
    average_cpu: f64,
    average_active_workers: f64,
    max_cpu: u32,
}

struct SimulationResult {
    name: String,
    total_completed: usize,
    cpu_completed: usize,
    io_completed: usize,
    makespan_ms: u128,
    average_wait_ms: f64,
    average_turnaround_ms: f64,
    max_wait_ms: u128,
    average_cpu: f64,
    worker_usage: f64,
    max_cpu: u32,
}
//FN MAIN START