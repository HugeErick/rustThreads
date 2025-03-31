// one linear gonna go :(
use std::{
    sync::{Arc, Mutex, Condvar},
    thread,
    time::Duration,
};

// Represents a conveyor belt with a limited capacity
struct ConveyorBelt {
    buffer: Mutex<Vec<String>>, // Shared buffer for storing items
    capacity: usize,            // Maximum capacity of the conveyor belt
    notFull: Condvar,           // Condition variable for when the belt is not full
    notEmpty: Condvar,          // Condition variable for when the belt is not empty
}

impl ConveyorBelt {
    // Creates a new conveyor belt with a given capacity
    fn new(capacity: usize) -> Self {
        Self {
            buffer: Mutex::new(Vec::new()),
            capacity,
            notFull: Condvar::new(),
            notEmpty: Condvar::new(),
        }
    }

    // Adds an item to the conveyor belt (producer action)
    fn addItem(&self, item: String, id: usize) {
        let mut buffer = self.buffer.lock().unwrap();

        // Wait if the conveyor belt is full
        while buffer.len() == self.capacity {
            println!("Producer {} waiting (belt full)", id);
            buffer = self.notFull.wait(buffer).unwrap();
        }

        // Add item to the buffer
        buffer.push(item.clone());
        println!("Producer {} added: {}", id, item);

        // Notify a waiting consumer that an item is available
        self.notEmpty.notify_one();
    }

    // Removes an item from the conveyor belt (consumer action)
    fn removeItem(&self, id: usize) -> String {
        let mut buffer = self.buffer.lock().unwrap();

        // Wait if the conveyor belt is empty
        while buffer.is_empty() {
            println!("Consumer {} waiting (belt empty)", id);
            buffer = self.notEmpty.wait(buffer).unwrap();
        }

        // Remove and return the first item from the buffer
        let item = buffer.remove(0);
        println!("Consumer {} took: {}", id, item);

        // Notify a waiting producer that space is available
        self.notFull.notify_one();
        item
    }
}

// Runs the conveyor belt simulation with multiple producers and consumers
pub fn conveyorBeltRunTest() {
    let belt = Arc::new(ConveyorBelt::new(5)); // Shared conveyor belt with capacity 5

    let mut handles = vec![];

    // Spawn 3 producer threads
    for i in 0..3 {
        let belt = belt.clone();
        handles.push(thread::spawn(move || {
            for j in 0..5 {
                belt.addItem(format!("Item {}-{}", i, j), i);
                thread::sleep(Duration::from_millis(200));
            }
        }));
    }

    // Spawn 2 consumer threads
    for i in 0..2 {
        let belt = belt.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..7 {
                belt.removeItem(i);
                thread::sleep(Duration::from_millis(300));
            }
        }));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

