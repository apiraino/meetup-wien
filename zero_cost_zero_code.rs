// Type your code here, or load an example.
pub trait Trans<S>
where S: State,
      Self: State,
{
    fn next(self) -> S;
}

pub trait End<S>
where
    S: State,
    Self: State,
{
    fn end(self) -> S;
}

pub trait State{}

pub struct Start;

impl State for Start {}

pub struct Next;

impl State for Next {}

impl Trans<Next> for Start {
    fn next(self) -> Next {
        Next {}
    }
}

pub struct EndState;

impl State for EndState {}

impl End<EndState> for Next {
    fn end(self) -> EndState {
        EndState {}
    }
}

pub fn do_it() {
    let start = Start {};
    let next: Next = start.next();
    let _ : EndState = next.end();
}