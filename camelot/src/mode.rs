#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    A,
    B,
}

impl Mode {
    pub fn from_str(input: &str) -> Result<Self, String> {
        match input {
            "a" | "A" => Ok(Self::A),
            "b" | "B" => Ok(Self::B),
            _ => Err(String::from("Invalid input.")),
        }
    }
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
