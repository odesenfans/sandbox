fn main() {
    let counter = ::std::sync::Arc::new(::std::sync::Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = ::std::sync::Arc::clone(&counter);
        let handle = ::std::thread::spawn(move || {
            let mut n = counter.lock().unwrap();
            *n += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
