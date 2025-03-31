mod menuOfProblems;
mod problems;

use menuOfProblems::menuOfProblems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  menuOfProblems()?;
  println!("Hello, world!");
  Ok(())
}
