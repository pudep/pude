use crate::prelude::normal::*;
use crate::render::render_line::render;
use crate::app_state::ViewPort;
pub fn app(
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
