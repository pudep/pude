pub mod cursor;
pub mod io;

#[derive(Debug)]
pub struct Buffer {
  /// The main data
  pub rope: Rope,
  /// the virtual cursor
  /// differs from Rope's (col, row)
  cursor: (u16, u16),
}
impl Default for Buffer {
  fn default() -> Self {
    Buffer {
      rope: Rope::new(),
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
}
