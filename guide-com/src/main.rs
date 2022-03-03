use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for n in &numbers[1..] {
        d = gcd(d, *n);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd<T>(mut n: T, mut m: T) -> T
where
    T: std::ops::Rem<Output = T> + std::cmp::PartialOrd + Ord + Into<usize> + From<usize>,
{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(20, 15), 5);
}
