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
    kind: TaskKind,
    duration_ms: u64,
    cpu_cost: u32,
    created_at: Instant,
}




    let mut total_wait = 0;
    let mut total_turnaround = 0;
    let mut max_wait = 0;
    let mut cpu_completed = 0;
    let mut io_completed = 0;



    fn print_result(result: &SimulationResult) {