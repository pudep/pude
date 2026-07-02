use std::{cell::OnceCell, error::Error, result};

pub fn read() -> result::Result<pude::state::buffer::Buffer, Box<dyn Error>> {
  let mut buf = pude::state::buffer::Buffer::new();
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
fn smart_soft_wrap(width: u16, text: &str) -> Vec<&str> {
  let options =
    textwrap::Options::new(width.into()).word_splitter(textwrap::WordSplitter::NoHyphenation);

  let wrapped_text: Vec<&str> = textwrap::wrap(text, options)
    .into_iter()
    .map(|cow| match cow {
      std::borrow::Cow::Borrowed(s) => s,
      std::borrow::Cow::Owned(_) => unreachable!("The text was unreachable"),
    })
    .collect();

  wrapped_text
}
fn main() -> result::Result<(), Box<dyn Error>>{
  let buf = read()?;

  for (idx, line) in buf.rope.lines().enumerate() {
    let line = line.to_string();
    let wrap = smart_soft_wrap(40, &line);
    println!("{:?}", wrap);
  }
  Ok(())
}
