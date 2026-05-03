# Concurrent Task Dispatcher

This project is a Rust simulation of a concurrent task dispatcher.

The program creates CPU and IO tasks, places them into queues, and sends them to a fixed worker pool. It compares FIFO scheduling with a simple optimized scheduling policy.

## How to Build

First go into the Rust project folder:

```bash
cd concurrent_task_dispatcher