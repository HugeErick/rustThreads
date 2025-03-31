use std::{sync::{Arc, Mutex, Condvar}, thread, time::Duration};

// Define a ReadWriteLock structure to manage multiple readers and a single writer
struct ReadWriteLock {
  readerCount: Mutex<usize>,   // Mutex-protected counter 
  writerActive: Mutex<bool>,   // "" boolean
  conditionVariable: Condvar,  // Condition variable for synchronization
}

impl ReadWriteLock {
  // "Constructor" for initializing the lock structure
  fn new() -> Self {
    ReadWriteLock {
      readerCount: Mutex::new(0),
      writerActive: Mutex::new(false),
      conditionVariable: Condvar::new(),
    }
  }

  // Function that allows a thread to perform a read operation
  fn readTime(&self, id: usize) {
    let mut readerCount = self.readerCount.lock().unwrap(); // Lock reader count

    // Wait while a writer is active
    while *self.writerActive.lock().unwrap() {
      readerCount = self.conditionVariable.wait(readerCount).unwrap();
    }
    *readerCount += 1; 
    drop(readerCount); // Release lock

    // Simulate reading process
    println!("Reader {} started reading...", id);
    thread::sleep(Duration::from_millis(500));
    println!("Reader {} finished reading...", id);

    // Decrement reader count after finishing
    let mut readerCount = self.readerCount.lock().unwrap();
    *readerCount -= 1;

    // If no readers are left, notify waiting writers
    if *readerCount == 0 {
      self.conditionVariable.notify_all();
    }
  }

  // Function that allows a thread to perform a write operation
  fn writeTime(&self, id: usize) {
    let mut writerActive = self.writerActive.lock().unwrap(); // Lock writer state

    // Wait while another writer is active or there are active readers
    while *writerActive || *self.readerCount.lock().unwrap() > 0 {
      writerActive = self.conditionVariable.wait(writerActive).unwrap();
    }

    *writerActive = true; // Mark writer as active
    drop(writerActive); // Release lock

    // Simulate writing process
    println!("Writer {} started writing...", id);
    thread::sleep(Duration::from_millis(500));
    println!("Writer {} finished writing...", id);

    // Reset writer state and notify waiting threads
    let mut writerActive = self.writerActive.lock().unwrap();
    *writerActive = false;
    self.conditionVariable.notify_all();
  }
}

// Function to test the ReadWriteLock with multiple reader and writer threads
pub fn readWriteRunTest() -> Result<(), Box<dyn std::error::Error>> {
  let readWriteLock = Arc::new(ReadWriteLock::new()); // Shared lock instance
  let mut handles = vec![];

  // Spawn 5 reader threads
  for i in 0..5 {
    let lock = readWriteLock.clone();
    handles.push(thread::spawn(move || {
      for _ in 0..3 { // Each reader reads 3 times
        lock.readTime(i);
      }
    }));
  }

  // Spawn 2 writer threads
  for i in 0..2 {
    let lock = readWriteLock.clone();
    handles.push(thread::spawn(move || {
      for _ in 0..2 { // Each writer writes 2 times
        lock.writeTime(i);
      }
    }));
  }

  for handle in handles {
    handle.join().unwrap();
  }

  Ok(())
}

