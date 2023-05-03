use crate::mode::Mode;

#[derive(Clone, Copy, PartialEq)]
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

impl Eq for Camelot {}

impl PartialOrd for Camelot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
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
