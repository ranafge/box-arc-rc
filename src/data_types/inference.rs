// The type inference engine is prety smart. 
pub fn inference() {
    // example of type inference
    let elem = 5u8;
    // create an empty vector (a groable array);
    // At this point compiler does not know the type of vector
    // just know its a vector of `vec<_>`
    let mut vec = Vec::new();

    // push an element to the vector
    vec.push(elem);
    // Aha! Now the compiler knows the  type of vec of u8;

    println!("{:?}", vec);
    
}