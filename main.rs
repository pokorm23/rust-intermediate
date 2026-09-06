use std::{cell::RefCell, io::{self, BufRead}, rc::Rc, sync::{Arc, Mutex}, thread};

fn main() -> io::Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut v = counter.lock().unwrap();
            *v+=250;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());

    Ok(())
}