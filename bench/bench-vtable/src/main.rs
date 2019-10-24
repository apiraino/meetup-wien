pub trait Draw {
    fn draw(&self);
}

pub struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button ...");
    }
}

// fn shoot(w: &dyn Draw) {
// }

pub fn main() {
    let w: &dyn Draw = &Button {};
    for _ in 0..1_000_000 {
        w.draw();
        // shoot(&Button {});
    }
}
