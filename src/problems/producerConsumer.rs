// this one liners are crazy 
use std::{sync::{Arc, Mutex, Condvar}, thread, time::Duration};

// rust way of doing this is to use const
impl TheBuffer {
  pub const SIZE: usize = 50;
}

//////////////////////////////////
// this is a queue now
//////////////////////////////////
pub struct TheBuffer {
    theData: [u8; Self::SIZE],
    head: usize,  
    tail: usize,  
    count: usize, 
}

pub fn producerConsumerRunTest() -> Result<(), Box<dyn std::error::Error>> {
    /*
    Arc is a reference counted pointer: i.e 
    enables multiple ownerships of the same data
    across threads
    Mutex is a mutex: i.e 
    enables mutual exclusion of the data
    across threads
    Condvar is a condition variable: i.e 
    enables waiting for a condition to be met
    across threads
     */
    let theBuffer = Arc::new((Mutex::new(TheBuffer {
        theData: [0; TheBuffer::SIZE],
        head: 0,
        tail: 0,
        count: 0,
    }), Condvar::new()));

    // Arc::clone creates a new reference to the same data
    // It increments the reference count but points to the same
    // underlying buffer and synchronization primitives
    // This allows multiple threads to safely share ownership
    let producerBuffer = Arc::clone(&theBuffer);
    let consumerBuffer = Arc::clone(&theBuffer);

    let producerHandle = thread::spawn(move || {
        let (lock, cvar) = &*producerBuffer;
        for i in 0..100 {
            let mut theBuffer = lock.lock().unwrap();
            // Wait while buffer is full
            while theBuffer.count == TheBuffer::SIZE {
                // Release lock and wait for consumer to make space
                theBuffer = cvar.wait(theBuffer).unwrap();
            }
            // Get current tail position
            let tail = theBuffer.tail;
            // Insert new data at tail
            theBuffer.theData[tail] = i as u8;
            // Move tail pointer, wrapping around if needed
            theBuffer.tail = (theBuffer.tail + 1) % TheBuffer::SIZE;
            // Increment count of items in buffer
            theBuffer.count += 1;
            println!("Produced: {}", i);
            // Wake up any waiting consumers
            cvar.notify_all();
            // Release lock i.e the Mutex lock (the restroom is free now)
            drop(theBuffer);
            thread::sleep(Duration::from_millis(50));
        }
    });

    let consumerHandle = thread::spawn(move || {
        let (lock, cvar) = &*consumerBuffer;
        for _ in 0..100 {
            let mut theBuffer = lock.lock().unwrap();
            // Wait while buffer is empty
            while theBuffer.count == 0 {
                // Release lock and wait for producer to add items
                theBuffer = cvar.wait(theBuffer).unwrap();
            }
            // Get current head position
            let head = theBuffer.head;
            // Retrieve data from head
            let data = theBuffer.theData[head];
            // Move head pointer, wrapping around if needed
            theBuffer.head = (theBuffer.head + 1) % TheBuffer::SIZE;
            // Decrement count of items in buffer
            theBuffer.count -= 1;
            println!("Consumed: {}", data);
            // Wake up any waiting producers
            cvar.notify_all();
            // We should drop the lock here to avoid holding it during sleep
            drop(theBuffer);
            thread::sleep(Duration::from_millis(100));
        }
    });

    producerHandle.join().unwrap();
    consumerHandle.join().unwrap();
    Ok(())
}
