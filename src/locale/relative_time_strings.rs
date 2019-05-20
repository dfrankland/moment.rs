#[derive(Debug, Clone)]
pub struct RelativeTime {
    pub future: &'static str,
    pub past: &'static str,
    pub s: &'static str,
    pub ss: &'static str,
    pub m: &'static str,
    pub mm: &'static str,
    pub h: &'static str,
    pub hh: &'static str,
    pub d: &'static str,
    pub dd: &'static str,
    pub M: &'static str,
    pub MM: &'static str,
    pub y: &'static str,
    pub yy: &'static str,
}

impl RelativeTime {
    pub fn get_by_key(&self, key: &str) -> Option<&'static str> {
        match key {
            "future" => Some(self.future),
            "past" => Some(self.past),
            "s" => Some(self.s),
            "ss" => Some(self.ss),
            "m" => Some(self.m),
            "mm" => Some(self.mm),
            "h" => Some(self.h),
            "hh" => Some(self.hh),
            "d" => Some(self.d),
            "dd" => Some(self.dd),
            "M" => Some(self.M),
            "MM" => Some(self.MM),
            "y" => Some(self.y),
            "yy" => Some(self.yy),
            _ => None,
        }
    }
}
