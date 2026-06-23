use pude::app::state;
fn main(){
  let mut buf = state::Buffer::new();
  buf.reader("Hello, World");

  println!("{:?}", buf);
}
