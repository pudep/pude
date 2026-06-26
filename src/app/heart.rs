pub fn life() -> Result<(), Box<dyn std::error::Error>> {
  loop {
    if crate::key::core::key_pressed()? {
      break;
    }
  }
  Ok(())
}
