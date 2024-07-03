use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Rc (Reference Counted) example
    println!("Rc Example:");
    rc_example();

    // Arc (Atomically Reference Counted) example
    println!("\nArc Example:");
    arc_example();

    // Mutex (Mutual Exclusion) example
    println!("\nMutex Example:");
    mutex_example();
}

// Rc: for shared ownership in single-threaded scenarios
fn rc_example() {
    let data = Rc::new(42);
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);

    println!("Original: {}", *data);
    println!("Clone 1: {}", *data_clone1);
    println!("Clone 2: {}", *data_clone2);
    println!("Reference count: {}", Rc::strong_count(&data));
}

// Arc: for shared ownership across multiple threads
fn arc_example() {
    let data = Arc::new(42);
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("In thread: {}", *data_clone);
    });

    println!("In main: {}", *data);
    handle.join().unwrap();
}

// Mutex: for safe mutable access in multi-threaded scenarios
fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread increment: {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
