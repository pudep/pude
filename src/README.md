Building a Terminal Text Editor with Crossterm 0.29

This guide walks you through building a minimal terminal-based text editor using crossterm version 0.29. We'll build incrementally, covering essential concepts like raw mode, event handling, and terminal manipulation.

Setup

Create a new Rust project and add the dependency:

```toml
[dependencies]
crossterm = "0.29"
```

Key features of crossterm 0.29 include event reading, terminal manipulation, clipboard support via OSC52, and improved event helper methods (is_*, as_*) for cleaner code .

1. Raw Mode

The terminal needs raw mode for immediate key responses instead of line buffering :

```rust
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn setup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    Ok(())
}

fn cleanup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    Ok(())
}
```

Raw mode changes terminal behavior: input won't be echoed, no line buffering, and special keys won't be processed by the terminal driver (use write! instead of println!) .

2. Reading Input Events

Poll for keyboard events with the event module :

```rust
use crossterm::event::{read, Event, KeyCode};

fn get_key_event() -> Result<Event, Box<dyn std::error::Error>> {
    loop {
        if let Event::Key(key_event) = read()? {
            return Ok(Event::Key(key_event));
        }
    }
}

// In your main loop
loop {
    match get_key_event()? {
        Event::Key(key) => {
            match key.code {
                KeyCode::Char('q') => break,     // Quit
                KeyCode::Char(c) => handle_char(c),
                KeyCode::Enter => handle_newline(),
                KeyCode::Backspace => handle_backspace(),
                KeyCode::Up => move_cursor_up(),
                KeyCode::Down => move_cursor_down(),
                KeyCode::Left => move_cursor_left(),
                KeyCode::Right => move_cursor_right(),
                _ => {}
            }
        }
        _ => {}
    }
}
```

v0.29 tip: Use is_* helper methods for cleaner event checks .

3. Terminal & Cursor Control

Clear the screen and control cursor position:

```rust
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveTo, MoveUp, MoveDown, MoveLeft, MoveRight};
use crossterm::execute;

// Clear screen and move cursor to top-left
execute!(
    stdout(),
    Clear(ClearType::All),
    MoveTo(0, 0)
)?;

// Move cursor relative to current position
execute!(
    stdout(),
    MoveUp(1),   // Move up one row
    MoveDown(1), // Move down one row
    MoveLeft(1), // Move left one column
    MoveRight(1) // Move right one column
)?;
```

4. Rendering

Render the editor state (file content, cursor position) each frame using queue! for batching :

```rust
use crossterm::{queue, style::Print, terminal::Clear, cursor::MoveTo};
use std::io::Write;

fn render(
    stdout: &mut std::io::Stdout,
    content: &[String],
    cursor_x: u16,
    cursor_y: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    queue!(stdout, Clear(ClearType::All))?;

    for (row, line) in content.iter().enumerate() {
        queue!(stdout, MoveTo(0, row as u16), Print(line))?;
    }

    queue!(stdout, MoveTo(cursor_x, cursor_y))?;
    stdout.flush()?;
    Ok(())
}
```

5. Editor State

Track the file content and cursor position:

```rust
struct Editor {
    content: Vec<String>,  // Lines of text
    cursor_x: u16,         // Column position
    cursor_y: u16,         // Row position
    row_offset: u16,       // For scrolling
    col_offset: u16,
}

impl Editor {
    fn new(initial_content: String) -> Self {
        Self {
            content: initial_content.lines().map(|s| s.to_string()).collect(),
            cursor_x: 0,
            cursor_y: 0,
            row_offset: 0,
            col_offset: 0,
        }
    }
}
```

6. Input Handling Implementation

```rust
impl Editor {
    fn handle_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') => return false, // Quit
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Enter => self.insert_newline(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            _ => {}
        }
        true
    }

    fn insert_char(&mut self, c: char) {
        if self.cursor_y < self.content.len() as u16 {
            let line = &mut self.content[self.cursor_y as usize];
            line.insert(self.cursor_x as usize, c);
            self.cursor_x += 1;
        }
    }

    fn delete_char(&mut self) {
        if self.cursor_y < self.content.len() as u16 {
            let line = &mut self.content[self.cursor_y as usize];
            if self.cursor_x > 0 {
                line.remove((self.cursor_x - 1) as usize);
                self.cursor_x -= 1;
            } else if self.cursor_y > 0 {
                // Join with previous line
                let prev_line = &mut self.content[self.cursor_y as usize - 1];
                let line_to_join = self.content.remove(self.cursor_y as usize);
                prev_line.push_str(&line_to_join);
                self.cursor_y -= 1;
                self.cursor_x = prev_line.len() as u16;
            }
        }
    }
}
```

7. Complete Main Loop

```rust
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    // Optional: Save terminal state on panic
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        panic_hook(panic_info);
    }));

    let mut editor = Editor::new("Hello, World!\nWelcome to your new editor.".to_string());
    let mut stdout = std::io::stdout();

    loop {
        render(&mut stdout, &editor.content, editor.cursor_x, editor.cursor_y)?;

        if let Event::Key(key) = read()? {
            if !editor.handle_input(key.code) {
                break;
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
```

v0.29-Specific Features

Clipboard Support (OSC52)

Crossterm 0.29 adds clipboard copy capabilities :

```rust
use crossterm::clipboard::{ClipboardProvider, ClipboardContext};
use crossterm::Result;

fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(text.to_string())?;
    Ok(())
}

fn paste_from_clipboard() -> Result<String> {
    let ctx = ClipboardContext::new()?;
    ctx.get_contents()
}
```

Improved Event Helpers

Use is_key(), is_mouse(), and as_key() for cleaner code :

```rust
if let Event::Key(key) = event {
    // Instead of manual matches, use:
    if event.is_key() {
        if let Some(key_event) = event.as_key() {
            // key_event is &KeyEvent
        }
    }
}
```

8. Panic Hook & Cleanup

Prevent the terminal from being left in raw mode or alternate screen on panic :

```rust
fn setup_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        // Restore original screen if using alternate screen
        hook(panic_info);
    }));
}
```

Next Steps

· Add syntax highlighting with crossterm::style::SetForegroundColor/SetBackgroundColor
· Implement scrolling with cursor::MoveTo and viewport offsets
· Add an alternate screen buffer with terminal::EnterAlternateScreen/LeaveAlternateScreen
· Support mouse events with event::EnableMouseCapture/DisableMouseCapture 

The key to building a text editor is the immediate feedback loop: raw mode for instant key handling, rendering the full screen state each frame, and maintaining an accurate editor state . Crossterm 0.29 provides all the low-level tools needed for this approach.
