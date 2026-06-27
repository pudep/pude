use crate::app;
use crate::prelude::cterm::all::*;
use crate::prelude::std::all::*;
fn disable_raw_mode() -> Result<(), Box<dyn std::error::Error>> {
  println!();
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

pub fn init() -> Result<(), Box<dyn Error>> {
  crossterm::terminal::enable_raw_mode()?;
  let mut stdout = stdout();
  app::loopin::main_loop(&mut stdout)?;
  disable_raw_mode()?;
  execute!(
    stdout,
    terminal::Clear(terminal::ClearType::All),
    cursor::MoveTo(0, 0)
  )?;
  Ok(())
}
