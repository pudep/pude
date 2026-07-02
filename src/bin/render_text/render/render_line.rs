use crate::app_state::*;
use crate::prelude::normal::*;
use crate::wrap::util::*;
pub fn render(frame: &mut Frame, rope: &Rope, viewport: &mut ViewPort) {
  let mut content = String::new();
  let terminal_area = frame.area();
  let mut llnw = rope.lines().len().to_string().len().max(5);
  let nw = push_space_char(llnw as u16);

  for (idx, line) in rope.lines().skip(viewport.scroll_offset).enumerate() {
    let line = line.to_string();
    let wrapped_text = smart_soft_wrap(terminal_area.width, &line);
    let line_number = idx + viewport.scroll_offset + 1;
    let wrapped_line = smart_soft_wrap(terminal_area.width, &line);
    content.push_str(&wrapped_line);
  }
  frame.render_widget(Paragraph::new(content), terminal_area);
}
