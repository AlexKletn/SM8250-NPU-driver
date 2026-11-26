#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltCheck {
    Poll,
    Delay,
    Voted,
}
