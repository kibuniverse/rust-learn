mod config;
mod modulea;
use modulea::add::add as addAs;
w
fn main() {
    config::print_config("hello rust module".to_string());
    let add = addAs(1, 2);
    println!("add: {}", add);
    modulea::print::print_hello_rust();
}
