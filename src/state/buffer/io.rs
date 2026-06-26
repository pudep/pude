use super::Buffer;
use std::{fs::File, io::BufReader, path::PathBuf};

impl Buffer {
  pub fn read(&mut self, path: PathBuf) -> Result<&mut Buffer, std::io::Error> {
    self.rope = ropey::Rope::from_reader(BufReader::new(File::open(path)?))?;
    Ok(self)
  }
}
