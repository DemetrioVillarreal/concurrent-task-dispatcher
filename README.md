# Concurrent Task Dispatcher

This project is a Rust simulation of a concurrent task dispatcher.

The program creates CPU and IO tasks, places them into queues, and sends them to a fixed worker pool. It compares FIFO scheduling with a simple optimized scheduling policy.

## How to Build

First go into the Rust project folder:

```bash
cd concurrent_task_dispatcher
```

Then build the project:

```bash
cargo build
```

## How to Run

Run this from inside the `concurrent_task_dispatcher` folder:

```bash
cargo run
```

## Command Examples

Build the project:

```bash
cd concurrent_task_dispatcher
cargo build
```

Run the project:

```bash
cd concurrent_task_dispatcher
cargo run
```

Save the experiment output:

```bash
cd concurrent_task_dispatcher
cargo run > experiment_output.txt
```

## Summary of Design

The program has four main parts: the task generator, the manager, the worker pool, and the monitor thread.

The task generator creates 1000 tasks using a fixed random seed. Each task is either an IO task or a CPU task. The generator sends one task every 20ms.

The manager receives tasks and places them into queues. For FIFO scheduling, it uses one queue. For optimized scheduling, it uses one CPU queue and one IO queue.

The worker pool has 8 worker threads. Each worker waits for a task, runs the task by sleeping for 200ms, and then sends a completion message back.

The monitor thread checks current CPU usage and active workers every 10ms.

The program uses channels to send tasks and completion messages. It also uses Arc and Mutex to protect shared state such as current CPU usage, active workers, and the done flag.

## Summary of Experiments

The program runs four experiments:

1. FIFO with 70% IO and 30% CPU
2. Optimized with 70% IO and 30% CPU
3. FIFO with 80% IO and 20% CPU
4. Optimized with 80% IO and 20% CPU

The important comparison is total runtime and average CPU usage.

For the 70/30 workload, the optimized scheduler finished faster and had better average CPU usage than FIFO.

For the 80/20 workload, FIFO and optimized were almost the same. This is because there were more IO tasks, so CPU pressure was lower and the optimized scheduler did not have much room to improve the result.

The full printed output is saved in:

```text
concurrent_task_dispatcher/experiment_output.txt
```

## Metrics Printed

The program prints:

- total tasks completed
- CPU tasks completed
- IO tasks completed
- total runtime / makespan
- average wait time
- average turnaround time
- max wait time
- average CPU usage
- max CPU usage
- average worker usage

## Tool Use Disclosure

I used a few tools while working on this project, including Rust documentation, Stack Overflow, Cargo compiler messages, Youtube tutorials, GitHub Codespaces, Git, GitHub, ChatGPT, and the Rust standard library documentation. The Rust documentation and compiler messages helped me fix syntax and build errors. GitHub Codespaces was used to write and run the project. Git and GitHub were used to save versions of the project and push the final files to the repository. ChatGPT helped me organize the project requirements, review my design, and check that my explanation matched the code.

One piece of advice I accepted was using channels to pass tasks and completion messages between the generator, manager, and workers. This made the task flow easier to follow and explain.

One piece of advice I changed was how much logging to print. I kept enough logging to show tasks moving into workers, but I avoided adding extra formatting or a larger logging system because the main focus of the project is the dispatcher and metrics.

