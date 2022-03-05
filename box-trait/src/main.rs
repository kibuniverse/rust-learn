

fn print_str(str: &str) {
    println!("the str is {}", str);
}


fn main() {
    let str = String::from("Hello World!");
    print_str(&str);
}
