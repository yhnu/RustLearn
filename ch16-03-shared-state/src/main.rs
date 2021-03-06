use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn main2() {
    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter2 = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter2.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
