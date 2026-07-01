use ratatui::{
  DefaultTerminal, Frame,
  layout::{
    Constraint::{self},
    Layout,
  },
  widgets::{Block, Borders, Paragraph},
};

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  ratatui::run(app)?;
  Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
  loop {
    terminal.draw(render)?;
    if crossterm::event::read()?.is_key_press() {
      break Ok(());
    }
  }
}

fn render(frame: &mut Frame) {
  let [top, bottom] =
    Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(frame.area());

  let [left, right] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(top);

  let [bleft, bright] =
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(bottom);
  // left pane: only right border (the vertical line)
  frame.render_widget(
    Paragraph::new("left").block(Block::new().borders(Borders::RIGHT)),
    left,
  );

  // right pane: no borders
  frame.render_widget(
    Paragraph::new("right").block(Block::new().borders(Borders::NONE)),
    right,
  );

  // bottom bar: only top border (the horizontal line)
  frame.render_widget(
    Paragraph::new("view3").block(Block::new().borders(Borders::TOP | Borders::RIGHT)),
    bleft,
  );

  frame.render_widget(
    Paragraph::new("view4").block(Block::new().borders(Borders::TOP)),
    bright,
  );
}
