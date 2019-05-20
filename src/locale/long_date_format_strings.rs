#[derive(Debug, Clone)]
pub struct LongDateFormat {
    pub LT: &'static str,
    pub LTS: &'static str,
    pub L: &'static str,
    pub LL: &'static str,
    pub LLL: &'static str,
    pub LLLL: &'static str,
}

impl LongDateFormat {
    pub fn get_by_key(&self, key: &str) -> Option<&'static str> {
        match key {
            "LT" => Some(self.LT),
            "LTS" => Some(self.LTS),
            "L" => Some(self.L),
            "LL" => Some(self.LL),
            "LLL" => Some(self.LLL),
            "LLLL" => Some(self.LLLL),
            _ => None,
        }
    }
}
