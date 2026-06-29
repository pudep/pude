use crate::prelude::cterm::all::*;
use ropey::Rope;
use std::io;

pub fn render_lines(rope: &Rope, stdout: &mut impl io::Write, cursor_pos: (u16, u16)) -> io::Result<()> {
  let (term_width, term_height) = terminal::size()?;
  let visible_lines = term_height.saturating_sub(1) as usize;
  let mut screen_row = 0u16;

  for line in rope.lines() {
    if screen_row as usize >= visible_lines {
      break;
    }
    queue!(stdout, cursor::MoveTo(0, screen_row))?;
    let mut line_text = String::new();
    for chunk in line.chunks() {
      line_text.push_str(chunk.trim_end_matches('\n').trim_end_matches('\r'));
    }
    queue!(stdout, style::Print(&line_text))?;
    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;

    // advance screen_row by how many terminal rows this line consumed
    let line_rows = (line_text.len() as u16).saturating_sub(1) / term_width + 1;
    screen_row += line_rows;
  }

  queue!(stdout, cursor::MoveTo(cursor_pos.0, cursor_pos.1))?;
  Ok(())
}
