pub fn push_space_char(width: u16) -> String {
  let mut x = width;
  let mut string = String::new();
  while x != 0 {
    string.push(' ');
    x -= 1;
  }
  string
}

pub fn smart_soft_wrap(width: u16, text: &str) -> String {
  let i = width.saturating_sub(5).max(3);
  let options =
    textwrap::Options::new(i as usize).word_splitter(textwrap::WordSplitter::NoHyphenation).word_separator(textwrap::WordSeparator::AsciiSpace);

  let wrapped_text: Vec<&str> = textwrap::wrap(text, options)
    .into_iter()
    .map(|cow| match cow {
      std::borrow::Cow::Borrowed(s) => s,
      std::borrow::Cow::Owned(_) => unreachable!("The text was unreachable"),
    })
    .collect();

  let mut content_line = String::new();
  for (idx, line) in wrapped_text.into_iter().enumerate() {
    if idx == 0 {
      let first_row = format!("  1. {line}\n");
      content_line.push_str(&first_row);
    } else if idx > 0 {
      let wrapped_row = format!("     {line}\n");
      content_line.push_str(&wrapped_row);
    }
  }

  content_line
}
