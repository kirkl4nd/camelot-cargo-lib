#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    A,
    B,
}

impl std::ops::Not for Mode {
    type Output = Mode;
    fn not(self) -> Self {
        match self {
            Mode::A => Mode::B,
            Mode::B => Mode::A,
        }
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::A => "A".to_string(),
            Mode::B => "B".to_string(),
        }
    }
}
