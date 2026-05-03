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

    let monitor_state = Arc::clone(&shared_state);

    let monitor_handle = thread::spawn(move || {
        let mut cpu_samples: Vec<u32> = Vec::new();
        let mut worker_samples: Vec<usize> = Vec::new();
        let mut max_cpu = 0;

        loop {
            thread::sleep(Duration::from_millis(10));

            let state = monitor_state.lock().unwrap();

            cpu_samples.push(state.current_cpu);
            worker_samples.push(state.active_workers);

            if state.current_cpu > max_cpu {
                max_cpu = state.current_cpu;
            }

            if state.done {
                break;
            }
        }

        let mut cpu_total = 0;
        for value in &cpu_samples {
            cpu_total += *value as usize;
        }

        let mut worker_total = 0;
        for value in &worker_samples {
            worker_total += *value;
        }

        let average_cpu = if cpu_samples.len() > 0 {
            cpu_total as f64 / cpu_samples.len() as f64
        } else {
            0.0
        };

        let average_active_workers = if worker_samples.len() > 0 {
            worker_total as f64 / worker_samples.len() as f64
        } else {
            0.0
        };

        MonitorResult {

            average_cpu,
            average_active_workers,
            max_cpu,
            
        }
    });

    let (task_sender, task_receiver) = mpsc::channel::<Task>();
    let generator_config = config.clone();

    let generator_handle = thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64(generator_config.seed);

        for id in 1..=generator_config.total_tasks {
            let random_number = rng.gen_range(0..100);

            let kind;
            let cpu_cost;

            if random_number < generator_config.io_percent {
                kind = TaskKind::IO;
                cpu_cost = 10;
            } else {
                kind = TaskKind::CPU;
                cpu_cost = 35;
            }

            let task = Task {
                id,
                kind,
                duration_ms: generator_config.duration_ms,
                cpu_cost,
                created_at: Instant::now(),
            };

            task_sender.send(task).unwrap();

            thread::sleep(Duration::from_millis(generator_config.interval_ms));
        }
    });


    let (complete_sender, complete_receiver) = mpsc::channel::<CompletedTask>();

    let mut worker_senders = Vec::new();

    let mut worker_handles = Vec::new();

    for worker_id in 0..config.workers {
        let (worker_sender, worker_receiver) = mpsc::channel::<Option<Task>>();
        worker_senders.push(worker_sender);

        let worker_complete_sender = complete_sender.clone();

        let handle = thread::spawn(move || {

            loop {

                let message = worker_receiver.recv().unwrap();

                match message {
                    Some(task) => {
                        let start = Instant::now();



                        println!(
                            "Worker {} started task {} ({:?})",
                            worker_id, task.id, task.kind
                        );

                        thread::sleep(Duration::from_millis(task.duration_ms));

                        let finish = Instant::now();

                        let completed = CompletedTask {
                            kind: task.kind,
                            wait_ms: start.duration_since(task.created_at).as_millis(),
                            turnaround_ms: finish.duration_since(task.created_at).as_millis(),
                            cpu_cost: task.cpu_cost,
                            worker_id,


                        };

                        worker_complete_sender.send(completed).unwrap();
                    }
                    None => {

                        break;
                    }

                }

            }

        });



        worker_handles.push(handle);

    }



    let mut fifo_queue: VecDeque<Task> = VecDeque::new();

    let mut cpu_queue: VecDeque<Task> = VecDeque::new();
    let mut io_queue: VecDeque<Task> = VecDeque::new();

    let mut available_workers: Vec<usize> = Vec::new();

    for i in 0..config.workers {
        available_workers.push(i);
    }

    let mut completed_tasks: Vec<CompletedTask> = Vec::new();

    while completed_tasks.len() < config.total_tasks {

        loop {
            match task_receiver.try_recv() {
                Ok(task) => {

                    match policy {
                        Policy::Fifo => {
                            fifo_queue.push_back(task);
                        }
                        Policy::Optimized => {

                            match task.kind {

                                TaskKind::CPU => cpu_queue.push_back(task),
                                TaskKind::IO => io_queue.push_back(task),
                            }

                        }
                    }

                }
                Err(mpsc::TryRecvError::Empty) => {
                    break;

                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }

        loop {

            match complete_receiver.try_recv() {
                Ok(done_task) => {
                    {


                        let mut state = shared_state.lock().unwrap();

                        if state.current_cpu >= done_task.cpu_cost {
                            state.current_cpu -= done_task.cpu_cost;
                        }

                        if state.active_workers > 0 {
                            state.active_workers -= 1;
                        }
                    }

                    available_workers.push(done_task.worker_id);
                    completed_tasks.push(done_task);

                }
                Err(mpsc::TryRecvError::Empty) => {
                    break;


                }
                Err(mpsc::TryRecvError::Disconnected) => {

                    break;

                }
            }
        }

        loop {
            if available_workers.len() == 0 {


                break;
            }

            let current_cpu = {

                let state = shared_state.lock().unwrap();
                state.current_cpu

                
            };