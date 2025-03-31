use crate::problems::{producerConsumer::producerConsumerRunTest, philosophersDining::philosophersDiningRunTest};
use std::io::{self, Write};

pub fn menuOfProblems() -> Result<(), Box<dyn std::error::Error>> {
  loop {
    println!("\nChoose the problem");
    println!("1. producer consumer.");
    println!("2. philosophers dining");
    println!("3.");
    println!("4.");
    println!("5.");
    println!("6.");
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
        break;
      }
      "4" => {
        println!("Opt 4:");
        break;
      }
      "5" => {
        println!("Opt 5:");
        break;
      }
      "6" => {
        println!("Opt 6:");
        break;
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
