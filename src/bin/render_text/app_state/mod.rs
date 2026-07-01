pub struct ViewPort {
  pub scroll_offset: usize,
}

impl ViewPort {
  pub fn new() -> Self {
    ViewPort { scroll_offset: 0 }
  }
}
