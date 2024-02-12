pub trait Vechicle {
    fn drive(&self);
}

struct Truck;

impl Vechicle for Truck {
    fn drive(&self) {
        println!("Truck is driving");
    }
}

fn main() {
    // Every this is in stack . stack is much faster than heap. The problem is compile time mush know the size
    let t: Box<dyn Vechicle>;
    // here in line below due to avoid the mismatch type need to wrap with Box::new. Now error gone and its working.
    t = Box::new(Truck);
    t.drive();
}
