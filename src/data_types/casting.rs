// Suppress all warnings from casts which overflow

pub fn casting () {

    let decimal = 65.4321_f32;
    // Error! No implicit converstion
    // let integer: u8 = decimal;
    // FIXME 

    // Explicit conversion
    let integer = decimal as u8;
    println!("The integer is {}", integer);

    // let character = decimal as char;


}
