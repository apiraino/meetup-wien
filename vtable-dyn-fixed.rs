use std::fmt::Debug;

pub trait Weapon {
    fn hit(&self, target: &dyn Debug);
}

pub struct Sword;

impl Weapon for Sword {
    fn hit(&self, target: &dyn Debug) {
        print!("Chopped {:?} clean in half", target);
    }
}

pub struct Samurai<'w> {
    weapon: &'w dyn Weapon,
}

impl<'w> Samurai<'w> {
    pub fn new(weapon: &'w dyn Weapon) -> Self {
        Samurai{ weapon }
    }
    
    pub fn attack(&self, target: &dyn Debug) {
        self.weapon.hit(target);
    }
}

pub fn main() {
    let Samurai = Samurai::new(&Sword{});
    Samurai.attack(&"Akuna Matata");
    Samurai.attack(&32);
}
