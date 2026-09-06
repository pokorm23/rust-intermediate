use std::{cell::RefCell, io::{self, BufRead}, rc::Rc};

mod geometry {
    pub fn circle_area(radius: f64) -> f64 {
        3.14 * radius.powi(2)
    }

    pub fn square_area(side: f64) -> f64 {
        side.powi(2)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;
    let kind = &s[0];
    let dim: f64 = s[1].trim().parse().unwrap();

     let area = if kind.trim() == "circle" {
         geometry::circle_area(dim)
    } else {
         geometry::square_area(dim)
    };

    println!("{:.2}", area);

    Ok(())
}