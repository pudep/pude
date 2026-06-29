use std::io::Write;

use crate::prelude::cterm::all::*;
use crate::prelude::std::all::*;
use crate::render::buffer::lines;
use crate::state::buffer::Buffer;
pub fn run() -> Result<(), Box<dyn Error>> {
  crossterm::terminal::enable_raw_mode()?;
  let mut stdout = stdout();
  let mut buffer = crate::state::buffer::init::init()?;
  engine(&mut buffer, &mut stdout)?;
  println!();
  crossterm::terminal::disable_raw_mode()?;
  execute!(
    stdout,
    terminal::Clear(terminal::ClearType::All),
    cursor::MoveTo(0, 0)
  )?;
  Ok(())
}

fn engine(buffer: &mut Buffer, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
  queue!(stdout, terminal::EnableLineWrap)?;
  loop {
    queue!(stdout, cursor::Hide, cursor::MoveTo(0,0))?;
    lines::render_lines(&buffer.rope, stdout, buffer.cursor)?;
    queue!(stdout, cursor::Show)?;
    stdout.flush()?;
    if crate::key::core::key_pressed(&mut buffer.cursor)? {
      break;
    }
  }
  Ok(())
}
