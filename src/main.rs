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
    capacity: i32
}

use std::rc::Rc;
fn main() {
    let (capacity_1, capacity_2, capacity_3) = (Rc::new(Truck{capacity: 1}),Rc::new( Truck {capacity:2}), Rc::new(Truck{capacity:3}));
    let truck_capacity_vec = vec![Rc::clone(&capacity_1), capacity_2];
    let truck_capacity_vec2 = vec![capacity_1, capacity_3];
    println!("vec capacity one {:?}, vec capacity two {:?}", truck_capacity_vec, truck_capacity_vec2);


}