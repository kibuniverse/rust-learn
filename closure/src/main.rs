extern crate student;
use student::*;

fn main() {
    let add = |a: i32, b: i32| -> i32 { a + b };
    let yankaizhi = Student::new("yankaizhi".to_string(), 22);
    yankaizhi.say_hello();
    println!("add {}", add(1, 2));
}
