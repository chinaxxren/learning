#![allow(unused)]
fn main() {
  trait ClickCallback {
    fn on_click(&self, x: i64, y: i64);
  }

  struct Button {
    listeners: Vec<Box<dyn ClickCallback>>,
  }
}