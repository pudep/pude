use crossterm::cursor;
use ropey::Rope;

#[derive(Debug)]
pub struct Buffer {
  bufdata: Rope,
  cursor: (u16, u16),
}

impl Default for Buffer {
  fn default() -> Self{
    Buffer {
      bufdata: Rope::new(),
      cursor: (0, 0),
    }
  }
}

impl Buffer {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn reader(&mut self, str: &str) -> &mut Self {
    self.bufdata = Rope::from_str(str);
    self
  }

  pub fn get_cursor_pos(&mut self) -> &mut Self {
    let (col, row) = cursor::position().unwrap();
    self.cursor = (col, row);
    self
  }
}
