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



fn main() {

    let config = Config {

        total_tasks: 1000,
        workers: 8,
        io_percent: 70,
        interval_ms: 20,
        duration_ms: 200,
        seed: 12345,

    };

    println!("Concurrent Task Dispatcher");
    println!("Tasks: {}", config.total_tasks);
    println!("Workers: {}", config.workers);
    println!("Workload: {}% IO / {}% CPU", config.io_percent, 100 - config.io_percent);
    println!();

    let fifo_result = run_simulation(config.clone(), Policy::Fifo);
    print_result(&fifo_result);

    println!();
    println!("----------------------------------------");
    println!();

    let optimized_result = run_simulation(config.clone(), Policy::Optimized);
    print_result(&optimized_result);

    println!();
    println!("Comparison:");
    println!("FIFO runtime: {} ms", fifo_result.makespan_ms);
    println!("Optimized runtime: {} ms", optimized_result.makespan_ms);
    println!("FIFO average CPU: {:.2}%", fifo_result.average_cpu);
    println!("Optimized average CPU: {:.2}%", optimized_result.average_cpu);
}

fn run_simulation(config: Config, policy: Policy) -> SimulationResult {

    let name = match policy {
        Policy::Fifo => "FIFO".to_string(),
        Policy::Optimized => "Optimized".to_string(),
    };

    println!("Starting {} simulation...", name);

    let start_time = Instant::now();

    let shared_state = Arc::new(Mutex::new(SharedState {

        current_cpu: 0,
        active_workers: 0,
        done: false,
    }));

    