fn handle_scroll(editor: &mut Buffer, key: KeyEvent) {
  let (_, term_height) = terminal::size().unwrap_or((80, 24));
  let visible_lines = term_height.saturating_sub(1) as usize;
  let total_lines = editor.rope.len_lines();
  let half_page = visible_lines / 2;

  match key.code {
    // Ctrl-d: half page down
    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
      editor.scroll_offset =
        (editor.scroll_offset + half_page).min(total_lines.saturating_sub(visible_lines));
      editor.cursor_row = (editor.cursor_row + half_page).min(total_lines - 1);
    }
    // Ctrl-u: half page up
    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
      editor.scroll_offset = editor.scroll_offset.saturating_sub(half_page);
      editor.cursor_row = editor.cursor_row.saturating_sub(half_page);
    }
    // j / Down: single line down
    KeyCode::Char('j') | KeyCode::Down => {
      if editor.cursor_row < total_lines - 1 {
        editor.cursor_row += 1;
        // scroll viewport down when cursor hits bottom
        let screen_row = editor.cursor_row - editor.scroll_offset;
        if screen_row >= visible_lines {
          editor.scroll_offset += 1;
        }
      }
    }
    // k / Up: single line up
    KeyCode::Char('k') | KeyCode::Up => {
      if editor.cursor_row > 0 {
        editor.cursor_row -= 1;
        // scroll viewport up when cursor hits top
        if editor.cursor_row < editor.scroll_offset {
          editor.scroll_offset -= 1;
        }
      }
    }
    _ => {}
  }
}
