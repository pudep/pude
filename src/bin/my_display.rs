use std::{error::Error, result};

use crossterm::{
  event::{Event, KeyCode, KeyModifiers},
  style::SetAttribute,
};
use pude::state;
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};
use ropey::Rope;

struct ViewPort {
  scroll_offset: usize,
}

impl ViewPort {
  fn new() -> Self {
    ViewPort { scroll_offset: 0 }
  }
}

/// my read
fn read() -> result::Result<state::buffer::Buffer, Box<dyn Error>> {
  let mut buf = state::buffer::Buffer::new();
  let path = std::env::home_dir()
    .unwrap()
    .join("impl")
    .join("rust")
    .join("pude")
    .join("src")
    .join("txt.txt");
  buf.read(path)?;
  Ok(buf)
}

fn main() -> result::Result<(), Box<dyn Error>> {
  let buf = read()?;
  let mut viewport = ViewPort::new();
  ratatui::run(|terminal| app(terminal, &buf.rope, &mut viewport))?;
  Ok(())
}

fn app(
  terminal: &mut DefaultTerminal,
  rope: &Rope,
  viewport: &mut ViewPort,
) -> std::io::Result<()> {
  loop {
    terminal.draw(|frame| render(frame, rope, viewport))?;
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          break Ok(());
        }

        KeyCode::Up => {
          viewport.scroll_offset += 1;
        }
        KeyCode::Down => {
          viewport.scroll_offset += 0;
        }
        _ => {}
      }
    }
  }
}

fn push_space_char(width: &mut usize) -> String {
  let mut string = String::new();
  while *width != 0 {
    string.push(' ');
    *width -= 1;
  }
  string
}

fn smart_soft_wrap(width: u16, text: &str) -> Vec<&str> {
  let options =
    textwrap::Options::new(width.into()).word_splitter(textwrap::WordSplitter::NoHyphenation);

  let wrapped_text: Vec<&str> = textwrap::wrap(text, options)
    .into_iter()
    .map(|cow| match cow {
      std::borrow::Cow::Borrowed(s) => s,
      std::borrow::Cow::Owned(_) => unreachable!("The text was unreachable"),
    })
    .collect();

  wrapped_text
}

fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let terminal_area = frame.area();
  let frame_height = terminal_area.height;
  let frame_width = terminal_area.width;

  let mut last_line_width = rope.lines().len().to_string().len().max(5);
  let number_width = push_space_char(&mut last_line_width);
  // let gutter_padding = Vec::new();
  let space = " ";

  let mut content = String::new();
  for (idx, line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let line = line.to_string();
    let wrapped_text = smart_soft_wrap(frame_width, &line);
    // idx == 0; scrolloffset say = 10 and current lie is 11 therefore 0 + 10 + extra 1 == 11!
    let line_number = idx + viewport.scroll_offset + 1;

    for (idx, line) in wrapped_text.iter().enumerate() {
      if idx == 0 {
        let wrap = format!("{space}{space}{line_number}{space}{line}");
        content.push_str(&wrap);
      } else if idx != 0 {
        let wrap = format!("{space}{space}{number_width}{space}{line}");
        content.push_str(&wrap);
      }
    }
  }

  frame.render_widget(Paragraph::new(content), terminal_area);
}
