// this one liners are crazy 
use std::{sync::{Arc, Mutex}, thread};

#[derive(Debug)]
struct Philosopher {
  id: usize,
  leftFork: Arc<(Mutex<()>, usize)>, // Reference-counted tuple: a mutex-protected fork and its unique ID
  rightFork: Arc<(Mutex<()>, usize)>, // Same as above, but for the right fork
}

impl Philosopher {
  fn eat(&self) {
    // To prevent deadlocks, philosophers always pick the fork with the lower ID first.
    let (first, second) = if self.leftFork.1 < self.rightFork.1 {
      (&self.leftFork, &self.rightFork) // Left fork has a lower ID, so it's picked first
    } else {
      (&self.rightFork, &self.leftFork) // Right fork has a lower ID, so it's picked first
    };

    // Lock the first fork (mutex). This ensures exclusive access to the fork.
    let _firstGuard = first.0.lock().unwrap(); // `first.0` accesses the `Mutex<()>` part of the tuple
    // Lock the second fork (mutex). This ensures the philosopher has both forks before eating.
    let _secondGuard = second.0.lock().unwrap(); // `second.0` accesses the `Mutex<()>` part of the tuple

    println!("Philosopher {} is eating", self.id); 
    // simulate eat yum yum nigg
    thread::sleep(std::time::Duration::from_secs(1)); 
  }
}

pub fn philosophersDiningRunTest() -> Result<(), Box<dyn std::error::Error>> {
  // Create 5 forks, each represented as a tuple containing a mutex-protected fork and a unique ID.
  let forks: Vec<_> = (0..5).map(|i| Arc::new((Mutex::new(()), i))).collect();
  // Each fork is wrapped in an `Arc` to allow shared ownership across multiple philosophers.

  // Create 5 philosophers, assigning each a left and right fork.
  let philosophers: Vec<_> = (0..5).map(|i| {
    let leftFork = forks[i].clone(); // The philosopher's left fork is the fork at index `i`
    let rightFork = forks[(i + 1) % 5].clone(); // The philosopher's right fork is the next fork in the circular table

    Philosopher {
      id: i, 
      leftFork, 
      rightFork,
    }
  }).collect();

  // println!("{:?}" , philosophers); // Uncomment this to print the list of philosophers for debugging purposes

  // Spawn a thread for each philosopher to simulate concurrent dining.
  let handles: Vec<_> = philosophers.into_iter().map(|p| {
    thread::spawn(move || {
      for _ in 0..5 { // Each philosopher eats 5 times
        p.eat(); 
      }
    })
  }).collect();

  // Wait for all philosopher threads to finish their execution.
  for handle in handles {
    handle.join().unwrap(); // Block the main thread until each philosopher thread completes
  }

  Ok(())
}
