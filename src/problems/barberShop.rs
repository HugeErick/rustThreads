use std::{sync::{Arc, Mutex, Condvar}, thread, time::{Duration}};

// Represents the barber shop with synchronization primitives
struct BarberShop {
  state: Mutex<BarberState>,     // Protected shared state
  barberReady: Condvar,          // Signals when barber is ready for next customer
  customerReady: Condvar,        // Signals when a customer arrives
}

// Internal state of the barber shop
struct BarberState {
  waitingCustomers: Vec<usize>,  // Queue of waiting customers by ID
  barberActive: bool,            // True when barber is cutting hair
  availableChairs: usize,        // Number of waiting chairs available
  open: bool, // a must if we want to return to menu
}

impl BarberShop {
  // Creates a new barber shop with specified number of waiting chairs
  fn new(chairs: usize) -> Self {
    BarberShop {
      state: Mutex::new(BarberState::new(chairs)),
      barberReady: Condvar::new(),
      customerReady: Condvar::new(),
    }
  }
}

impl BarberState {
  // Creates a new barber state with specified number of chairs
  fn new(chairs: usize) -> Self {
    BarberState {
      waitingCustomers: Vec::new(),
      barberActive: false,
      availableChairs: chairs,
      open: true,
    }
  }
}

// Simulates the barber's behavior in the shop
fn barberThread(shop: Arc<BarberShop>) {
  loop {
    let mut state = shop.state.lock().unwrap();

    while state.waitingCustomers.is_empty() {
      if !state.open {
        println!("Barber is going home, no more customers!");
        return;  // Exit when the shop is closed and no customers remain
      }
      println!("Barber sleeping zzzz");
      state = shop.customerReady.wait(state).unwrap();
    }

    // Get the next customer in line
    let customer = state.waitingCustomers.remove(0);
    state.barberActive = true;
    drop(state);

    println!("Barber cutting hair of {}", customer);
    thread::sleep(Duration::from_millis(500));

    // Finish with current customer
    let mut state = shop.state.lock().unwrap();
    state.barberActive = false;
    state.availableChairs += 1;
    println!("Barber finished with customer {}", customer);
    shop.barberReady.notify_all();
  }
}

// Simulates a customer's behavior in the shop
fn customerThread(shop: Arc<BarberShop>, id: usize) {
  let mut state = shop.state.lock().unwrap();

  // Leave if no chairs are available
  if state.availableChairs == 0 {
    println!("Customer {} left (no chairs)", id);
    return;
  }

  // Take a seat and wait
  state.waitingCustomers.push(id);
  state.availableChairs -= 1;
  println!("Customer {} sat down ({} chairs left)", id, state.availableChairs);

  // Wake up the barber if they're sleeping
  if state.waitingCustomers.len() == 1 && !state.barberActive {
    println!("Customer {} wakes up the barber!", id);
  }
  shop.customerReady.notify_all();

  // Wait until it's this customer's turn
  while state.barberActive || state.waitingCustomers.first() != Some(&id) {
    state = shop.barberReady.wait(state).unwrap();

    if state.waitingCustomers.is_empty() {
      return;
    }
  }

  state.availableChairs += 1;
  println!("Customer {} getting a haircut", id);
}

// Runs the barber shop simulation
pub fn BarberShopRunTest() -> Result<(), Box<dyn std::error::Error>> {
  let shop = Arc::new(BarberShop::new(3));
  let barberHandler = thread::spawn({
    let shop = shop.clone();
    move || barberThread(shop)
  });

  // Create a vector to store customer thread handles
  let mut customerHandles = vec![];

  // Spawn 7 customers
  for customerId in 0..7 {
    let shop = shop.clone();
    let handle = thread::spawn(move || {
      thread::sleep(Duration::from_millis(300));
      customerThread(shop, customerId);
    });
    customerHandles.push(handle);
  }

  // Wait for all customers to finish
  for handle in customerHandles {
    handle.join().unwrap();
  }

  {
    let mut state = shop.state.lock().unwrap();
    state.open = false;
    shop.customerReady.notify_all();
  }

  barberHandler.join().unwrap();

  Ok(())
}
