use std::io::{self, BufRead};

trait Shape { fn area(&self) -> f64; }
struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius.powi(2) * 3.14
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;

    let total: i32 = s[0].split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .filter(|x| *x % 2 == 0)
    .map(|x| x.pow(2))
    .sum::<i32>();

    println!("{}", total);

    Ok(())
}