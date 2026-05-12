use std::sync::{Arc, Mutex};
use std::thread;

const THREAD_COUNT: u64 = 4;
const INCREMENTS_PER_THREAD: u64 = 1_000_000;
static mut UNSAFE_COUNTER: u64 = 0;

fn part1_true_data_race_demo() {
    let expected = THREAD_COUNT * INCREMENTS_PER_THREAD;
    unsafe {
        UNSAFE_COUNTER = 0;
    }
    let mut handles = Vec::new();

    for _ in 0..THREAD_COUNT {
        let handle = thread::spawn(|| {
            for i in 0..INCREMENTS_PER_THREAD {
                // Intentionally unsafe shared mutable access without synchronization.
                // This creates a true data race and is undefined behavior.
                unsafe {
                    let current = UNSAFE_COUNTER;
                    UNSAFE_COUNTER = current + 1;
                }

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

    let actual = unsafe { UNSAFE_COUNTER };
    println!("Part 1: True data race demonstration (unsafe global counter)");
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
    part1_true_data_race_demo();
    part2_mutex_solution();
}
