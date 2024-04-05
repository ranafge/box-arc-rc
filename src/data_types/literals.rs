pub fn literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1_i32;
    let y = 33322232_i32;
    let z = 333333223_u64;
    // Unsuffixed literals, their types depend on how they are used

    let i = 1;
    let f  = 1.0; // 8 bytes

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}