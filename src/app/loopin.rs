use crate::prelude::cterm::all::*;
use crate::prelude::std::all::*;

use crate::render::{self};
pub fn main_loop(stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
  loop {
    execute!(
      stdout,
      terminal::Clear(terminal::ClearType::All),
      cursor::MoveTo(0, 0)
    )?;
    render::buffer::init::render_line(stdout)?;
    if crate::key::core::key_pressed()? {
      break;
    }
  }
  Ok(())
}
