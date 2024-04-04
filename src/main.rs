use core::fmt;
use std::{fmt::Display, sync::Arc};

pub trait Vechicle {
    fn drive(&self);
}

// struct Truck;

impl Vechicle for Truck {
    fn drive(&self) {
        println!("Truck is driving");
    }
}

// fn main() {
//     // Every this is in stack . stack is much faster than heap. The problem is compile time mush know the size
//     let t: Box<dyn Vechicle>;
//     // here in line below due to avoid the mismatch type need to wrap with Box::new. Now error gone and its working.
//     t = Box::new(Truck);
//     t.drive();
// }

// Box is used in another data type like recrusive

// #[derive(Debug)]
// struct Truck{
//     //optiion is used to repetaed value which mean next truck can either contain a truck or be none
//     // for this reason it is recursive
//     // to resolve the above problem use Box data type
//     next_truck: Option<Box<Truck>>
// }

// fn main() {
//     let t = Some(Box::new(Truck {next_truck: None}));

//     // we can use match statement for the next_truck field becuse its contain OPTION ENUM
//     match Some(t) {
//         Some(v) => println!("{:?}", v),
//         None => println!("none")
//     }

// }

#[derive(Debug)]
struct Truck {
    capacity: i32,
}

#[derive(Debug)]
enum CustomError {
    CannotBeZero
}

fn add_custom_error(a: i32, b:i32) -> Result<i32, CustomError> {
    if a == 0 || b == 0 {
        return Err(CustomError::CannotBeZero);
    }

    Ok(a + b)
}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::CannotBeZero => write!(f, "Cannot be zero number provie"),
        }
    }
}
fn main() {
    let (truck_a, truck_b, truck_c) = (
       Arc::new(Truck { capacity: 1 }),
       Arc::new(Truck { capacity: 2 }),
       Arc::new(Truck { capacity: 3 }),
    );
    let facility_one = vec![Arc::clone(&truck_a), Arc::clone(&truck_b)];
    // here truck_b is move below
    // USE Arc::CLONE OF TRUCK_B 
    let facility_two = vec![Arc::clone(&truck_b), Arc::clone(&truck_c)];

    let thread = std::thread::spawn(|| {
        println!("Facility one vec {:?}", facility_one);
        println!("Facility one vec {:?}", facility_two);
        (facility_one, facility_two)
    });
    let facility_thead = thread.join().unwrap();

    // println!("Facility one vec {:?}", facility_one);
    // println!("Facility one vec {:?}", facility_two);
    println!("strong count {:?}", Arc::strong_count(&truck_b));
    println!("strong count {:?}", Arc::strong_count(&truck_b));
    // std::mem::drop(facility_two);
    // println!("Facility one after drop facility tow {:?}", facility_one);
    println!("strong count {:?}", Arc::strong_count(&truck_b));
    let result = add_custom_error(4, 0);
    match result {
        Ok(value) => println!("The sum is {}", value),
        Err(e) => println!("An error occurred: {:?}", e),
    };
}
