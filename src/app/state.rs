use std::io::Read;

use crossterm::cursor;
use ropey::Rope;

#[derive(Debug)]
pub struct Buffer {
  /// The main data
  bufdata: Rope,
  /// the virtual cursor
  /// differs from Rope's (col, row)
  cursor: (u16, u16),
}

impl Default for Buffer {
  fn default() -> Self {
    Buffer {
      bufdata: Rope::new(),
      cursor: (0, 0),
    }
  }
}

impl Buffer {
  /// Initialize new Rope.
  /// Call once per file only!
  pub fn new() -> Self {
    Self::default()
  }

  /// Read calls Rope::from_reader() 
  /// Call once per file only!
  /// Feed a io::BufReader for preformance
  pub fn read<R: Read>(&mut self, mut reader: R) -> Result<&mut Buffer, std::io::Error> {
    self.bufdata = Rope::from_reader(&mut reader)?;
    Ok(self)
  }

  /// gives (col, row) of virtual/crossterm's 
  /// terminal cursor
  pub fn get_virtual_cursor_pos(&mut self) -> &mut Self {
    let (col, row) = cursor::position().unwrap();
    self.cursor = (col, row);
    self
  }

  fn clamp_col(&mut self){
    
  }

  /// Takes &mut Buffer and digit: u16
  /// Could be buggy!
  pub fn cursor_move_left(&mut self, digit: u16) -> &mut Self {
    if self.cursor.0 > 0 {
      self.cursor.0 -= digit;
    }
    self
  }

  /// Takes &mut Buffer and digit: u16
  /// moves cursor x digits right
  /// Could be buggy!
  pub fn move_cursor_left(&mut self, digit: u16) -> &mut Self {
    if self.cursor.0 != 0 || (self.cursor.0 - digit) > 0 {
      self.cursor.0 -= digit;
    }
    self
  }
}
