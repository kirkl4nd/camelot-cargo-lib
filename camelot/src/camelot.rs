use crate::mode::Mode;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct Camelot {
    key: u8,
    mode: Mode,
}

impl Camelot {
    // CONSTRUCTORS //

    pub fn new(key: u8, mode: Mode) -> Result<Self, String> {
        if key < 1 || key > 12 {
            return Err(String::from("Invalid key."));
        }
        return Ok(Self {
            key: key,
            mode: mode,
        });
    }

    pub fn from_str(input: &str) -> Result<Self, String> {
        let key_str: String = input.chars().filter(|char| char.is_ascii_digit()).collect();
        let key = key_str
            .parse::<u8>()
            .map_err(|_| String::from("Invalid key."))?;

        let mode_str: String = input
            .chars()
            .filter(|char| char.is_ascii_alphabetic())
            .collect();
        let mode = Mode::from_str(&mode_str)?;

        Ok(Self::new(key, mode)?)
    }

    // METHODS //

    fn get_compatible(&self) -> Vec<Camelot> {
        let mut results = Vec::new();

        // Add self, relative mode, +1, -1
        results.push(self.clone());
        results.push(Camelot::new(self.key, !self.mode).unwrap());
        results
            .push(Camelot::new(if self.key == 1 { 12 } else { self.key - 1 }, self.mode).unwrap());
        results
            .push(Camelot::new(if self.key == 12 { 1 } else { self.key - 1 }, self.mode).unwrap());

        results.sort();

        return results;
    }
}

impl Ord for Camelot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.key.cmp(&other.key);
    }
}

impl ToString for Camelot {
    fn to_string(&self) -> String {
        return format!("{:2}{}", self.key, self.mode.to_string());
    }
}
