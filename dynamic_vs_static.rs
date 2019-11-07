// Type your code here, or load an example.
use std::fmt::Debug;

pub trait Weapon<T>
where T: Debug {
    fn hit(&self, target: T);
}

// equals to:
// trait Weapon {
//     fn hit(&self, target: impl Debug);
// }

pub struct Sword;

impl<T> Weapon<T> for Sword
where T: Debug {
    fn hit(&self, target: T) {
        println!("Chopped {:?} clean in half", target);
    }
}

pub struct Samuray<'w, T> where T: Debug {
    weapon: Box<dyn Weapon<T> + 'w>,
}

impl<'w, T> Samuray<'w, T>
where T: Debug {
    pub fn new(weapon: impl Weapon<T> + 'w) -> Self {
        Samuray{ weapon: Box::new(weapon) }
    }
    
    pub fn attack(&self, target: T) {
        self.weapon.hit(target);
    }
}

pub fn main() {
    let samuray = Samuray::new(Sword{});
    samuray.attack("Akuna Matata");
    //samuray.attack(32);
}