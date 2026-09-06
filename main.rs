use std::io::{self, BufRead};

fn append_excl(s: &mut String) { 
    s.push('!');
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;

    println!("{}", longer(&s[0], &s[1]));

    Ok(())
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}