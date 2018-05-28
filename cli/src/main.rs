use std::io::Write;
use std::str::FromStr;

fn main() {
    println!("Running cli()");

    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: input a list of numbers ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = d + *m;
    }

    println!("The sum of your input of {:?} is {}", numbers, d);
}
