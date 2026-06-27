mod app;
mod key;
mod prelude;
mod render;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  app::init::init()?;
  Ok(())
}
