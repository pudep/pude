mod app;
mod key;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  app::init::init()?;
  Ok(())
}
