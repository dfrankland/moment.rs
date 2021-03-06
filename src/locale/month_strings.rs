#[derive(Debug, Clone)]
pub struct MonthStrings(
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
);

impl MonthStrings {
    pub fn get_index(&self, index: u32) -> Option<&'static str> {
        match index {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            3 => Some(self.3),
            4 => Some(self.4),
            5 => Some(self.5),
            6 => Some(self.6),
            7 => Some(self.7),
            8 => Some(self.8),
            9 => Some(self.9),
            10 => Some(self.10),
            11 => Some(self.11),
            _ => None,
        }
    }
}
