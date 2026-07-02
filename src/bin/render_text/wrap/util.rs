pub fn push_space_char(width: &mut usize) -> String {
  let mut string = String::new();
  while *width != 0 {
    string.push(' ');
    *width -= 1;
  }
  string
}

pub fn smart_soft_wrap(width: u16, text: &str) -> String {
  let options =
    textwrap::Options::new(width.into()).word_splitter(textwrap::WordSplitter::NoHyphenation);

  let wrapped_text = textwrap::wrap(text, options).join("\n");
  wrapped_text
}
