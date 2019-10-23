// Type your code here, or load an example.
pub trait Weapon {
    fn features(&self);
}

pub struct Bomb;

impl Weapon for Bomb {
    fn features(&self) {
        println!("Killing humanity in progress...");
    }
}

fn shoot<T: Weapon>(w: T) {
    w.features();
}

pub fn do_it() {
    shoot(Bomb{});
}