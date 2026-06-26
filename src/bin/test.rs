use std::{
  env,
  error::Error,
  io::{self, stdout},
};

use crossterm::{
  cursor,
  event::{self, Event, KeyCode},
  execute,
  terminal::{self, ClearType},
};
use pude::{app::rope_state, state};
use ropey::Rope;

fn main() -> Result<(), Box<dyn Error>> {
  let mut rope = state::buffer::Buffer::new();
  let path = env::home_dir()
    .expect("unable to find home dir.")
    .join("impl")
    .join("rust")
    .join("exec")
    .join("src")
    .join("txt.txt");
  rope.read(path)?;

  terminal::enable_raw_mode()?;
  let mut stdout = stdout();
  execute!(
    stdout,
    terminal::Clear(ClearType::All),
    cursor::MoveTo(0, 0)
  )?;

  render(&rope.rope, &mut stdout)?;

  loop {
    if let Event::Key(key) = event::read()?
      && key.code == KeyCode::Char('q')
    {
      break;
    }
  }
  println!();
  terminal::disable_raw_mode()?;
  execute!(
    stdout,
    terminal::Clear(ClearType::All),
    cursor::MoveTo(0, 0)
  )?;

  Ok(())
}

fn render(rope: &Rope, stdout: &mut impl io::Write) -> io::Result<()> {
  let (_, term_height) = terminal::size()?;
  for (i, lines) in rope.lines().enumerate() {
    if i as u16 > term_height {
      break;
    }
    execute!(
      stdout,
      cursor::MoveTo(0, i as u16),
      crossterm::style::Print(lines.to_string().trim_end_matches('\n'))
    )?;
  }
  stdout.flush()?;
  Ok(())
}
