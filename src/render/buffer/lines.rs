use crate::prelude::cterm::all::*;
use ropey::Rope;
use std::io;

pub fn render_lines(rope: &Rope, stdout: &mut impl io::Write) -> io::Result<()> {
  let (_, term_height) = terminal::size()?;
  let visible_lines = term_height.saturating_sub(1) as usize;

  for (screen_row, line) in rope.lines().take(visible_lines).enumerate() {
    queue!(stdout, cursor::MoveTo(0, screen_row as u16))?;
    for chunk in line.chunks() {
      let trimmed = chunk.trim_end_matches('\n').trim_end_matches('\r');
      queue!(stdout, style::Print(trimmed))?;
    }
    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
  }
  stdout.flush()
}
