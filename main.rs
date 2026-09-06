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

    let kind = &s[0];
    let dim = &s[1].parse::<f64>().unwrap();

    let s: Box<dyn Shape> = if kind == "circle" {
        Box::new(Circle { radius: *dim })
    } else {
        Box::new(Square { side: *dim })
    };

    println!("{:.2}", s.area());

    Ok(())
}