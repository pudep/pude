use std::{
  error::Error,
  fs,
  io::{BufRead, BufReader, stdout},
};

use crossterm::{execute, style::Print};

pub struct Data {
  string: Vec<String>,
}

impl Data {
  pub fn new() -> Self {
    Data {string: Vec::new()}
  }
  pub fn push(&mut self, s: String){
    self.string.push(s);
  }
  pub fn display(&self) -> String {
    self.string.join("\n")
  }
}

pub fn reader(data: &mut Data) -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    let path = format!("{}/txt.txt", home);
    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }
    Ok(())
}

pub fn print_it(data: &Data) {
  execute!(stdout(), Print(data.display()));
}
