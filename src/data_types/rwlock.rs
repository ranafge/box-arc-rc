//Rwlock means `read write lock`. It is like a mutex but also like a refcell.
// use .write().unwrap() instead of .lock().unwrap() to change it.
// can also use .read().unwrap() to get read access.
// It is like Refcell because it follows the rules:
// * many .read() variable is okay,
// * .wrete() variable is okay
// .write() or .read() together with .write() is not okay

// The program will run forever if you try to .write() when you can't get access:

pub fn rwlock() {
    use std::sync::RwLock;
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap(); // one .read() is fine
    let read2 = my_rwlock.read().unwrap(); // two .read() is also fine
    // println!("{:?} {:?}", read1, read2 );
    // let write1 = my_rwlock.write().unwrap(); // now the programm will wait forever
    // std::mem::drop(read1);
    // std::mem::drop(read2);
    // let mut write1 = my_rwlock.write().unwrap();
    // *write1 += 10;
    // println!("{:?}", *write1);
    // we can also try_read or try_write

    if let Ok(mut number) = my_rwlock.try_write(){
        *number += 10;
        println!("Now number is {}", number);
    }else {
        println!( "Couldn't acquire write lock");
    };

}