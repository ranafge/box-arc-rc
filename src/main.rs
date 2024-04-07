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



mod data_types;

use data_types::casting::casting;
use data_types::literals::literals;
use data_types::inference::inference;
use data_types::rwlock::rwlock;
fn main() {
    println!("{:?}", rwlock());
    println!("{:?}", casting());
    println!("{}", 33322232_i32);
    println!("{}", 333333223_u64);
    println!("{}", 222323_i32 as u128);
    println!("{:?}", literals());
    println!("{:?}", inference());

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
