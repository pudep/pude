use std::{error::Error, io::stdout, thread::{sleep, sleep_ms}, time::Duration};

use crossterm::{
  cursor,
  event::{Event, KeyCode, KeyModifiers},
};

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

fn get_cursor_position() -> (u16, u16) {
  let mut stdout = stdout();
  let (col, row) = cursor::position().unwrap();
  (col, row)
}

fn key_pressed() -> Result<bool, Box<dyn std::error::Error>> {
  match get_event_key()? {
    Event::Key(key) => match key.code {
      KeyCode::Char('q') => {
        if key.modifiers == KeyModifiers::CONTROL {
          return Ok(true);
        } else {
          println!("Insert char")
        }
      }
      _ => {}
    },
    _ => {}
  }
  Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
  setup_terminal()?;
  loop {
    if key_pressed()? {
      break;
    }
  }
  cleanup_terminal()?;
  Ok(())
}
