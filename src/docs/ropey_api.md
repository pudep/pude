# The Rope API — Essential Knowledge for Text Editor Developers

> A complete field guide to `ropey` for anyone building a text editor, from
> scratch or otherwise. Language-agnostic concepts, Rust-specific API.

---

## Why a rope and not a `String`

A `String` is one contiguous allocation. Every insert or delete in the middle
requires shifting every byte that follows it. On a 10 MB file, pressing a single
key can move megabytes of memory.

A rope stores text as a balanced tree of small string chunks. Insert and delete
become pointer surgery — O(log n) regardless of where in the document you are.

```
String:   [H][e][l][l][o][,][ ][w][o][r][l][d]
           ↑ insert here = shift everything right of cursor

Rope:          (node, weight=7)
              /                \
     "Hello, "               "world"
      insert here = split leaf, no shifting
```

The tradeoff: random character access is O(log n) instead of O(1). For an
editor that mostly reads sequential lines to render, that cost is negligible.

---

## The four types you will use

```rust
use ropey::{Rope, RopeSlice};
use ropey::iter::{Chars, Lines, Chunks, Bytes};
```

| Type | What it is |
|------|------------|
| `Rope` | The owned document. Lives as long as you need it. |
| `RopeSlice<'a>` | A borrowed view into a range of a `Rope`. Zero-copy. |
| `Chunks<'a>` | Iterator over the rope's internal `&str` nodes. |
| `Lines<'a>` | Iterator over lines as `RopeSlice`. |

The most important thing to internalize early: **`RopeSlice` is to `Rope` what
`&str` is to `String`**. Every read-only API that accepts a rope should take
`RopeSlice`, not `&Rope`.

---

## Chapter 1 — Creating a rope

```rust
// Empty document
let mut rope = Rope::new();

// From a string literal
let rope = Rope::from_str("Hello, world!\n");

// From a file (the efficient way — streams into the rope)
use std::io::BufReader;
use std::fs::File;
let file = BufReader::new(File::open("document.txt")?);
let rope = Rope::from_reader(file)?;
```

`from_reader` is what you want in production. It never loads the whole file into
a `String` first — it builds the rope directly from the buffered byte stream.

---

## Chapter 2 — The coordinate systems

This is where most editor bugs come from. Ropey gives you three coordinate
systems and you must never mix them up.

```
Text:          H  e  l  l  o  ,     w  o  r  l  d  \n  L  i  n  e  2  \n
Byte index:    0  1  2  3  4  5  6  7  8  9  10 11 12  13 14 15 16 17 18
Char index:    0  1  2  3  4  5  6  7  8  9  10 11 12  13 14 15 16 17 18
Line index:    |←————————————— line 0 ————————————————→|←—— line 1 ————→|
```

For ASCII text, byte index and char index are identical. The moment you have
Unicode — emoji, accented characters, CJK — they diverge. `é` is 1 char,
2 bytes. `😀` is 1 char, 4 bytes.

**Rule:** your editor's cursor is a **char index**, not a byte index. Ropey's
edit API (`insert`, `remove`) takes char indices. Use byte indices only when
talking to external systems (tree-sitter, language servers, network protocols)
that explicitly say they operate in bytes.

### The five conversion functions

```rust
// char ↔ byte
rope.char_to_byte(char_idx)   // char index → byte index
rope.byte_to_char(byte_idx)   // byte index → char index

// char ↔ line
rope.char_to_line(char_idx)   // char index → which line number
rope.line_to_char(line_idx)   // line number → char index of line start

// byte ↔ line
rope.byte_to_line(byte_idx)   // byte index → which line number
rope.line_to_byte(line_idx)   // line number → byte index of line start
```

### Deriving (line, column) from a char index

Ropey has no native (line, col) type. You compute it yourself:

```rust
fn char_to_line_col(rope: &Rope, char_idx: usize) -> (usize, usize) {
    let line = rope.char_to_line(char_idx);
    let line_start = rope.line_to_char(line);
    let col = char_idx - line_start;
    (line, col)
}

fn line_col_to_char(rope: &Rope, line: usize, col: usize) -> usize {
    rope.line_to_char(line) + col
}
```

Column here is in chars from line start. If your UI renders with a fixed-width
font, this is what you want. If you need display columns (where a tab counts
as N spaces, or a CJK char is 2 cells wide), you must walk the chars yourself
and accumulate display width.

---

## Chapter 3 — Length queries

```rust
rope.len_chars()   // total characters
rope.len_bytes()   // total bytes
rope.len_lines()   // total lines (including a partial last line with no \n)
rope.is_empty()    // true if len_chars() == 0
```

`len_lines()` counts a trailing newline as ending a line, not starting a new
one. A rope containing `"foo\n"` has 1 line. A rope containing `"foo\nbar"` has
2 lines. A rope containing `"foo\nbar\n"` has 2 lines. This matches how editors
conventionally count lines.

---

## Chapter 4 — Reading text

### The whole document

```rust
let s: String = rope.to_string();  // allocates, use sparingly
```

### A single line

```rust
let line: RopeSlice = rope.line(line_idx);
let line_str: String = line.to_string();

// Strip the trailing newline if present
let line_str = line.to_string();
let line_str = line_str.trim_end_matches('\n');
```

### A range of chars

```rust
// rope.slice(range) → RopeSlice, zero-copy
let word: RopeSlice = rope.slice(7..12);
let selection: RopeSlice = rope.slice(start_char..end_char);

// Whole rope as a slice (common when your API takes RopeSlice)
let whole: RopeSlice = rope.slice(..);
```

### A single character

```rust
rope.char(char_idx)   // → char. panics if out of bounds.

// Safe version:
if char_idx < rope.len_chars() {
    let c = rope.char(char_idx);
}
```

### Iterating characters

```rust
// All chars from the start
for c in rope.chars() { }

// Chars starting at a specific position (efficient — no skipping)
for c in rope.chars_at(char_idx) { }
```

`chars_at` is the right tool for syntax highlighting: start the iterator at the
first visible character, then pull chars until you've covered the visible range.

### Iterating lines

```rust
for line in rope.lines() {
    // line is a RopeSlice
    println!("{}", line);
}

// Lines starting at a line index
for line in rope.lines_at(first_visible_line) { }
```

---

## Chapter 5 — Editing

All edit operations take **char indices**, not byte indices.

### Insert

```rust
rope.insert(char_idx, "text");     // insert a string
rope.insert_char(char_idx, '\n');  // insert a single char (slightly faster)
```

Both are O(log n). The rope rebalances its internal tree automatically — you
never call a rebalance function manually.

### Remove

```rust
rope.remove(start_char..end_char);  // delete a char range [start, end)
```

Delete a single character at `idx`:
```rust
rope.remove(idx..idx + 1);
```

Backspace (delete char before cursor at `idx`):
```rust
if idx > 0 {
    rope.remove(idx - 1..idx);
}
```

Delete an entire line (including its newline):
```rust
let line_start = rope.line_to_char(line_idx);
let line_end = rope.line_to_char(line_idx + 1);  // start of next line = end of this one
rope.remove(line_start..line_end);
```

### Split and append

```rust
// Split into two ropes at a char index
let right = rope.split_off(char_idx);
// rope now contains [0, char_idx), right contains [char_idx, end)

// Append another rope (consumes it)
rope.append(right);
```

`split_off` and `append` are O(log n) — the rope doesn't copy text, it
rearranges tree nodes.

---

## Chapter 6 — Chunks: the low-level API

Most editor code never needs `chunks()`. You need it when you're building a
bridge to something that expects contiguous bytes — a syntax highlighter, a
hash function, a codec, or `io::Read`.

```rust
// Iterate over the rope's internal &str nodes
for chunk in rope.chunks() {
    // chunk: &str, borrowed from the rope's internals
    // No allocation. No copy.
    process(chunk.as_bytes());
}

// Same, but on a RopeSlice
for chunk in rope.line(5).chunks() { }

// Chunks starting at a char index
let (mut chunks, chunk_char_idx, _, _) = rope.chunks_at_char(char_idx);
// chunk_char_idx: char index of the start of the first returned chunk
```

`chunks_at_char` returns a 4-tuple: the chunk iterator, and the char/byte/line
index of where that first chunk starts in the document. You need these offsets
if you're mapping chunk positions back to document positions.

The canonical use of chunks — implementing `io::Read`:

```rust
use std::io;
use ropey::iter::Chunks;
use ropey::RopeSlice;

pub struct RopeReader<'a> {
    current_chunk: &'a [u8],
    chunks: Chunks<'a>,
}

impl<'a> RopeReader<'a> {
    pub fn new(slice: RopeSlice<'a>) -> Self {
        Self { current_chunk: &[], chunks: slice.chunks() }
    }
}

impl io::Read for RopeReader<'_> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        loop {
            let n = self.current_chunk.read(buf)?;
            buf = &mut buf[n..];
            if buf.is_empty() { return Ok(buf_len); }
            match self.chunks.next() {
                Some(chunk) => self.current_chunk = chunk.as_bytes(),
                None => return Ok(buf_len - buf.len()),
            }
        }
    }
}
```

This pattern appears in Helix, Lapce, and every serious Rust editor. It streams
the rope into any `io::Read` consumer without any allocation.

---

## Chapter 7 — Building an undo stack

Ropey's `Rope` is a value type — it implements `Clone`. Cloning a rope is
O(log n), not O(n), because the internal tree nodes are reference-counted and
shared between clones until one of them is mutated (copy-on-write).

The simplest correct undo stack:

```rust
pub struct History {
    snapshots: Vec<Rope>,
    current: usize,
}

impl History {
    pub fn new(initial: Rope) -> Self {
        Self { snapshots: vec![initial], current: 0 }
    }

    pub fn commit(&mut self, rope: Rope) {
        // Truncate any redo history
        self.snapshots.truncate(self.current + 1);
        self.snapshots.push(rope);
        self.current += 1;
    }

    pub fn undo(&mut self) -> Option<&Rope> {
        if self.current > 0 {
            self.current -= 1;
            Some(&self.snapshots[self.current])
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&Rope> {
        if self.current + 1 < self.snapshots.len() {
            self.current += 1;
            Some(&self.snapshots[self.current])
        } else {
            None
        }
    }
}
```

Usage:

```rust
let mut buf = Rope::from_str("hello");
let mut history = History::new(buf.clone());

buf.insert(5, " world");
history.commit(buf.clone());   // snapshot after edit

if let Some(prev) = history.undo() {
    buf = prev.clone();         // restore previous state
}
```

Because rope clones share tree nodes, a history of 100 snapshots uses far less
memory than 100 full `String` copies.

For a more sophisticated approach (batching keystrokes into transactions,
storing diffs instead of snapshots), see the `xi-rope` transaction model or
Helix's `history.rs`.

---

## Chapter 8 — The patterns that trip people up

### Off-by-one on line_to_char

```rust
// line_to_char gives the char index of the FIRST char on that line.
// To get the char index PAST the last char (exclusive end), use the next line.
let line_start = rope.line_to_char(line_idx);
let line_end   = rope.line_to_char(line_idx + 1); // includes the \n
let line_end_no_newline = line_end - 1;            // excludes the \n
```

But be careful: the last line of the document may not have a `\n`. Guard it:

```rust
fn line_end_char(rope: &Rope, line_idx: usize) -> usize {
    let last_line = rope.len_lines() - 1;
    if line_idx >= last_line {
        rope.len_chars()   // no newline at end of file
    } else {
        rope.line_to_char(line_idx + 1) - 1
    }
}
```

### len_lines() on an empty rope

```rust
let rope = Rope::new();
rope.len_lines()   // → 1, not 0
```

An empty rope is one empty line. Treat `len_lines()` as "number of lines that
exist to display", not "number of newline characters".

### Char indices into a RopeSlice are relative

```rust
let slice = rope.slice(10..20);  // chars 10–19 of the rope
let c = slice.char(0);           // this is rope.char(10), not rope.char(0)
```

When you get an offset back from a `RopeSlice` iterator, it's relative to the
slice, not the original rope. Add back the slice's start to get the absolute
char index.

### Mixing byte and char indices

The most common real-world source of bugs. Tree-sitter gives byte offsets.
Language Server Protocol gives UTF-16 code unit offsets. Ropey works in chars
(Unicode scalar values / UTF-8 codepoints).

```rust
// tree-sitter gave you a byte range → convert to chars
let start_char = rope.byte_to_char(ts_start_byte);
let end_char   = rope.byte_to_char(ts_end_byte);

// LSP gave you a UTF-16 offset → there is no built-in conversion.
// Walk the line's chars and count UTF-16 code units manually.
fn utf16_to_char(rope: &Rope, line: usize, utf16_col: usize) -> usize {
    let line_start = rope.line_to_char(line);
    let mut utf16_count = 0;
    for (i, c) in rope.chars_at(line_start).enumerate() {
        if utf16_count >= utf16_col { return line_start + i; }
        utf16_count += c.len_utf16();
    }
    line_start + rope.line(line).len_chars()
}
```

---

## Quick reference card

```
CREATE
  Rope::new()                    empty
  Rope::from_str(s)              from &str
  Rope::from_reader(r)           from io::Read (preferred for files)

LENGTHS
  .len_chars()                   char count
  .len_bytes()                   byte count
  .len_lines()                   line count
  .is_empty()

CONVERT COORDINATES
  .char_to_byte(i)               char → byte
  .byte_to_char(i)               byte → char
  .char_to_line(i)               char → line number
  .line_to_char(i)               line number → char index of line start
  .byte_to_line(i)               byte → line number
  .line_to_byte(i)               line number → byte index of line start

READ
  .char(i)                       single char at index
  .line(i)                       → RopeSlice of line i
  .slice(a..b)                   → RopeSlice of char range
  .to_string()                   → owned String (allocates)
  .chars()                       char iterator from start
  .chars_at(i)                   char iterator from char index
  .lines()                       line iterator
  .lines_at(i)                   line iterator from line index
  .chunks()                      &str chunk iterator (zero-copy)
  .chunks_at_char(i)             chunk iterator from char index

EDIT (all take char indices)
  .insert(i, s)                  insert &str at char index
  .insert_char(i, c)             insert char at char index
  .remove(a..b)                  delete char range [a, b)
  .split_off(i)                  split → returns right half
  .append(other)                 append another Rope

CLONE
  .clone()                       O(log n), copy-on-write tree nodes
```

---

## Further reading

- [`ropey` docs](https://docs.rs/ropey) — the authoritative API reference
- [Helix `rope_reader.rs`](https://github.com/helix-editor/helix/blob/master/helix-core/src/rope_reader.rs) — `io::Read` adapter
- [Helix `history.rs`](https://github.com/helix-editor/helix/blob/master/helix-core/src/history.rs) — transaction-based undo
- [Xi editor rope design doc](https://xi-editor.io/docs/rope_science_00.html) — the theory behind ropes in editors
- [Tree-sitter + ropey integration](https://github.com/tree-sitter/tree-sitter/blob/master/lib/binding_rust/README.md) — byte offset bridging

