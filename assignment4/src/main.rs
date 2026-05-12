use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const THREAD_COUNT: u64 = 4;
const INCREMENTS_PER_THREAD: u64 = 1_000_000;

fn part1_lost_update_demo() {
    let expected = THREAD_COUNT * INCREMENTS_PER_THREAD;
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..THREAD_COUNT {
        let shared_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for i in 0..INCREMENTS_PER_THREAD {
                // Intentionally incorrect pattern: load + store can lose updates.
                let current = shared_counter.load(Ordering::Relaxed);
                shared_counter.store(current + 1, Ordering::Relaxed);

                if i % 1_000 == 0 {
                    thread::yield_now();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("A thread panicked in Part 1");
    }

    let actual = counter.load(Ordering::Relaxed);
    println!("Part 1: Lost update demonstration (incorrect atomic usage)");
    println!("Expected result: {}", expected);
    println!("Actual result: {}", actual);
    println!("Race condition demonstrated: {}", actual != expected);
    println!();
}

fn part2_mutex_solution() {
    let expected = THREAD_COUNT * INCREMENTS_PER_THREAD;
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = Vec::new();

    for _ in 0..THREAD_COUNT {
        let shared_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..INCREMENTS_PER_THREAD {
                let mut value = shared_counter.lock().expect("Mutex poisoned");
                *value += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("A thread panicked in Part 2");
    }

    let actual = *counter.lock().expect("Mutex poisoned");
    println!("Part 2: Mutex synchronization");
    println!("Expected result: {}", expected);
    println!("Actual result: {}", actual);
    println!("Synchronization successful: {}", actual == expected);
}

fn main() {
    part1_lost_update_demo();
    part2_mutex_solution();
}
