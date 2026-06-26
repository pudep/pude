use super::Buffer;
use crossterm::cursor;

impl Buffer {
  pub fn get_virtual_cursor_pos(&mut self) -> &mut Self {
    let (col, row) = cursor::position().unwrap();
    self.cursor = (col, row);
    self
  }

  pub fn cursor_move_left(&mut self) -> &mut Self {
    if self.cursor.0 > 0 {
      self.cursor.0 -= 1;
    }
    self
  }

  pub fn move_cursor_right(&mut self) -> &mut Self {
    self.cursor.0 += 1;
    self
  }
}
