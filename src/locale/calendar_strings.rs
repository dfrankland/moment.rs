use super::CalendarFormat;

#[derive(Debug, Clone)]
pub struct Calendar {
    pub same_day: &'static str,
    pub next_day: &'static str,
    pub next_week: &'static str,
    pub last_day: &'static str,
    pub last_week: &'static str,
    pub same_else: &'static str,
}

impl Calendar {
    pub fn get_format_for_calendar_format(&self, calendar_format: CalendarFormat) -> &'static str {
        match calendar_format {
            CalendarFormat::SameDay => self.same_day,
            CalendarFormat::NextDay => self.next_day,
            CalendarFormat::NextWeek => self.next_week,
            CalendarFormat::LastDay => self.last_day,
            CalendarFormat::LastWeek => self.last_week,
            CalendarFormat::SameElse => self.same_else,
        }
    }
}
