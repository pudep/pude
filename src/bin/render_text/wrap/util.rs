pub fn push_space_char(width: &mut usize) -> String {
  let mut string = String::new();
  while *width != 0 {
    string.push(' ');
    *width -= 1;
  }
  string
}

pub fn smart_soft_wrap(width: u16, text: &str) -> Vec<&str> {
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
