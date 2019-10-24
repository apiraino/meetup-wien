// Type your code here, or load an example.
pub trait Draw {
    fn draw(&self);

    // what happens when we uncomment this?
    // fn draw2(&self){}
}

pub struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button ...");
    }
}

fn shoot(w: &dyn Draw) {
    w.draw();
}

pub fn do_it() {
    shoot(&Button{});
}