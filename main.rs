use std::io::{self, BufRead};

fn parse_two(a:&str,b:&str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;

    Ok(x + y)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;

    let total = parse_two(&s[0], &s[1])
      .map_err(|x| "error: invalid input")
      .map(|x| format!("sum: {}", x).to_string());

    let b = match parse_two(&s[0], &s[1]) {
        Ok(n) => format!("sum: {}", n),
        Err(e) => "error: invalid input".to_string()
    };

    println!("{}", b);

    Ok(())
}