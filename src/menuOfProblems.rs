use crate::problems::{
  barberShop::BarberShopRunTest,
  conveyorBelt::conveyorBeltRunTest,
  philosophersDining::philosophersDiningRunTest,
  processesShifts::shiftProcessesRunTest,
  producerConsumer::producerConsumerRunTest,
  readerWriter::readWriteRunTest,
};
use std::io::{self, Write};

pub fn menuOfProblems() -> Result<(), Box<dyn std::error::Error>> {
  loop {
    println!("\nChoose the problem");
    println!("1. producer consumer.");
    println!("2. philosophers dining");
    println!("3. reader writer");
    println!("4. barber shop");
    println!("5. conveyor belt");
    println!("6. processes shifts");
    println!("Exit");
    println!("Enter ur choice: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
      "1" => {
        println!("u choosed option 1:");
        producerConsumerRunTest()?;
      }
      "2" => {
        println!("Opt 2:");
        philosophersDiningRunTest()?;
      }
      "3" => {
        println!("Opt 3:");
        readWriteRunTest()?;
      }
      "4" => {
        println!("Opt 4:");
        BarberShopRunTest()?;
      }
      "5" => {
        println!("Opt 5:");
        conveyorBeltRunTest();
      }
      "6" => {
        println!("Opt 6:");
        shiftProcessesRunTest();
      }
      "exit"|"7"|"Exit" => {
        println!("Bye");
        break;
      }
      _ => {
        println!("Invalid option srry");
        break;
      }
    }
  }
  Ok(())
}
