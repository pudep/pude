use crate::app_state::*;
use crate::prelude::normal::*;
use crate::wrap::util::*;
pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let terminal_area = frame.area();
  let frame_height = terminal_area.height;
  let frame_width = terminal_area.width;

  let mut last_line_width = rope.lines().len().to_string().len().max(5);
  let number_width = push_space_char(&mut last_line_width);
  // let gutter_padding = Vec::new();
  let space = " ";

  let mut content = String::new();
  for (idx, line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let line = line.to_string();
    let wrapped_text = smart_soft_wrap(frame_width, &line);
    // idx == 0; scrolloffset say = 10 and current lie is 11 therefore 0 + 10 + extra 1 == 11!
    let line_number = idx + viewport.scroll_offset + 1;

    for (idx, line) in wrapped_text.iter().enumerate() {
      if idx == 0 {
        let wrap = format!("{space}{space}{line_number}{space}{line}");
        content.push_str(&wrap);
      } else if idx != 0 {
        let wrap = format!("{space}{space}{number_width}{space}{line}");
        content.push_str(&wrap);
      }
    }
  }
  frame.render_widget(Paragraph::new(content), terminal_area);
}
