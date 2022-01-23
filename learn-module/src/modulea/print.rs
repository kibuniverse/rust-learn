use super::add::add as add_as;
pub fn print_hello_rust() {
  println!("Hello, Rust module!");
  println!(" add 1, 2 = {}", add_as(1, 2));
}
