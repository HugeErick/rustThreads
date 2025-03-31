use std::{
    sync::{Arc, Mutex, Condvar},
    thread,
    time::Duration,
};

// Structure to manage alternating print shifts
struct ShiftPrinter {
    turn: Mutex<bool>,  // Mutex to track whose turn it is (true = Process A, false = Process B)
    condVar: Condvar,   // Condition variable to synchronize turns
}

impl ShiftPrinter {
    // Creates a new ShiftPrinter instance
    fn new() -> Self {
        Self {
            turn: Mutex::new(true),
            condVar: Condvar::new(),
        }
    }

    // Process A's function to print its message
    fn printA(&self) {
        let mut turn = self.turn.lock().unwrap();

        // Wait until it's Process A's turn
        while !*turn {
            turn = self.condVar.wait(turn).unwrap();
        }

        // Print message and switch turn
        println!("Process A: Working...");
        thread::sleep(Duration::from_millis(300));

        *turn = false;
        self.condVar.notify_one(); // Notify Process B
    }

    // Process B's function to print its message
    fn printB(&self) {
        let mut turn = self.turn.lock().unwrap();

        // Wait until it's Process B's turn
        while *turn {
            turn = self.condVar.wait(turn).unwrap();
        }

        // Print message and switch turn
        println!("Process B: Working...");
        thread::sleep(Duration::from_millis(300));

        *turn = true;
        self.condVar.notify_one(); // Notify Process A
    }
}

// Runs the alternating processes simulation
pub fn shiftProcessesRunTest() {
    let printer = Arc::new(ShiftPrinter::new()); // Shared ShiftPrinter instance

    let printerA = printer.clone();
    let printerB = printer.clone();

    // Spawn Process A thread
    let handleA = thread::spawn(move || {
        for _ in 0..5 {
            printerA.printA();
        }
    });

    // Spawn Process B thread
    let handleB = thread::spawn(move || {
        for _ in 0..5 {
            printerB.printB();
        }
    });

    // Wait for both threads to finish
    handleA.join().unwrap();
    handleB.join().unwrap();
}

