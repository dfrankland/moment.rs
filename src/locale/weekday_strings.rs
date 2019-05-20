#[derive(Debug, Clone)]
pub struct WeekDayStrings(
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
);

impl WeekDayStrings {
    pub fn get_by_index(&self, index: u32) -> Option<&'static str> {
        match index {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            3 => Some(self.3),
            4 => Some(self.4),
            5 => Some(self.5),
            6 => Some(self.6),
            _ => None,
        }
    }
}
