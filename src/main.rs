mod app;
use std::{error::Error, io::stdout};

use crossterm::{
  cursor,
  event::{Event, KeyCode, KeyModifiers},
  execute,
  style::Print,
};

use crate::app::state;

fn setup_terminal() -> Result<(), Box<dyn Error>> {
  crossterm::terminal::enable_raw_mode()?;
  Ok(())
}

fn cleanup_terminal() -> Result<(), Box<dyn Error>> {
  println!("");
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

fn get_event_key() -> Result<Event, Box<dyn Error>> {
  loop {
    if let Event::Key(key_event) = crossterm::event::read()? {
      return Ok(Event::Key(key_event));
    }
  }
}

fn key_pressed() -> Result<bool, Box<dyn Error>> {
  let mut stdout = stdout();
  match get_event_key()? {
    Event::Key(key) => match key.code {
      KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
        return Ok(true);
      }
      KeyCode::Backspace => {
        let (col, _) = cursor::position().unwrap();
        if col > 0 {
          execute!(stdout, cursor::MoveLeft(1), Print(' '), cursor::MoveLeft(1))?;
        }
      }
      KeyCode::Left => {
        execute!(stdout, cursor::MoveLeft(1))?;
      }
      KeyCode::Right => {
        execute!(stdout, cursor::MoveRight(1))?;
      }
      KeyCode::Enter => {
        execute!(stdout, Print("\r\n"))?;
      }
      KeyCode::Up => {
        execute!(stdout, cursor::MoveUp(1))?;
      }
      KeyCode::Down => {
        execute!(stdout, cursor::MoveDown(1))?;
      }
      KeyCode::Char(random_char) => {
        execute!(stdout, Print(random_char))?;
      }
      _ => {}
    },
    _ => {}
  }
  Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut data = app::state::Data::new();
  state::reader(&mut data)?;
  state::print_it(&data);

  setup_terminal()?;
  loop {
    if key_pressed()? {
      break;
    }
  }
  cleanup_terminal()?;
  Ok(())
}
