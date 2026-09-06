use std::{cell::RefCell, io::{self, BufRead}, rc::Rc};

fn parse_two(a:&str,b:&str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;

    Ok(x + y)
}

fn main() -> io::Result<()> {
    //let stdin = io::stdin();
    //let s: Vec<String> = stdin.lock().lines().collect::<Result<_,_>>()?;

    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;

    println!("{}", counter.borrow());

    Ok(())
}