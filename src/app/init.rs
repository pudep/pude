use crate::app;
use std::error::Error;
/// A clean wrapper arround crossterm::terminal::disable_raw_mode()
/// Removes the % sign that displays at the end of program.
fn disable_raw_mode() -> Result<(), Box<dyn std::error::Error>> {
  // must have no "" inside its parameter brace
  println!();
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

pub fn init() -> Result<(), Box<dyn Error>> {
  crossterm::terminal::enable_raw_mode()?;
  app::heart::life()?;
  disable_raw_mode()?;
  Ok(())
}
