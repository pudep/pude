use crate::prelude::normal::*;
pub fn read() -> result::Result<state::buffer::Buffer, Box<dyn Error>> {
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
