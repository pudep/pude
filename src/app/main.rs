use crate::prelude::cterm::all::*;
use crate::prelude::std::all::*;
use crate::render::{self};
use crate::state::buffer::Buffer;

pub fn run() -> Result<(), Box<dyn Error>> {
  crossterm::terminal::enable_raw_mode()?;
  let mut stdout = stdout();
  let buffer = crate::state::buffer::init::init()?;
  engine(&buffer, &mut stdout)?;
  println!();
  crossterm::terminal::disable_raw_mode()?;
  execute!(
    stdout,
    terminal::Clear(terminal::ClearType::All),
    cursor::MoveTo(0, 0)
  )?;
  Ok(())
}

fn engine(buffer: &Buffer, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
  loop {
    execute!(
      stdout,
      terminal::Clear(terminal::ClearType::All),
      cursor::MoveTo(0, 0)
    )?;
    render::buffer::lines::render_lines(&buffer.rope, stdout)?;
    if crate::key::core::key_pressed()? {
      break;
    }
  }
  Ok(())
}
