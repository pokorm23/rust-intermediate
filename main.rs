use std::{cell::RefCell, io::{self, BufRead}, rc::Rc};

fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;

    let n = s[0].split_whitespace().next().unwrap().parse::<usize>().unwrap();

    let mut c = make_counter();

    for _ in 0..n {
        println!("{}", c());
    }

    Ok(())
}