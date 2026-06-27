use ropey::Rope;

pub mod cursor;
pub mod io;

#[derive(Debug)]
pub struct Buffer {
  /// The main data
  pub rope: Rope,
  /// the virtual cursor
  /// differs from Rope's (col, row)
  pub cursor: (u16, u16),

  pub cursor_row: usize,
  pub scroll_offset: usize,
}
impl Default for Buffer {
  fn default() -> Self {
    Buffer {
      rope: Rope::new(),
      cursor: (0, 0),
      cursor_row: 0,
      scroll_offset: 0,
    }
  }
}

impl Buffer {
  /// Initialize new Rope.
  /// Call once per file only!
  pub fn new() -> Self {
    Self::default()
  }
}
