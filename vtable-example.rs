// Type your code here, or load an example.
pub trait Weapon {
    fn features(&self);
}

pub struct Bomb;

impl Weapon for Bomb {
    fn features(&self) {
        println!("Loading Bomb features");
    }
}

fn shoot(w: &dyn Weapon) {
    w.features();
}

pub fn do_it() {
    shoot(&Bomb{});
}