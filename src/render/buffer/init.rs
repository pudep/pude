use crate::prelude::std::all::*;
use crate::render;

pub fn render_line(stdout: &mut impl io::Write) -> io::Result<()> {
  let mut buffer = crate::state::buffer::Buffer::new();
  let path = env::home_dir()
    .expect("unable to find home dir.")
    .join("impl")
    .join("rust")
    .join("exec")
    .join("src")
    .join("txt.txt");
  buffer.read(path)?;
  render::buffer::lines::render_lines(&buffer.rope, stdout)?;
  Ok(())
}
