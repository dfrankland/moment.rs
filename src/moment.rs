use crate::{locale::{Locale, CalendarFormatFn, Calendar}, locales::LOCALE_EN_US, UnitOfTime, duration};
use chrono::{prelude::*, Duration};
use std::{collections::HashSet, ops::Deref, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Moment<T: TimeZone + Debug> {
    date_time: DateTime<FixedOffset>,
    time_zone: T,
    locale: Locale,
}

impl<T: TimeZone + Debug> Deref for Moment<T> {
    type Target = DateTime<FixedOffset>;

    fn deref(&self) -> &Self::Target {
        &self.date_time
    }
}

impl Moment<Local> {
    pub fn new() -> Result<Moment<Local>, String> {
        let now = Local::now();
        let time_zone = now.timezone();
        Ok(Moment {
            time_zone,
            date_time: now
                .with_timezone(&time_zone.offset_from_utc_datetime(&Utc::now().naive_utc())),
            locale: LOCALE_EN_US.clone(),
        })
    }

    pub fn utc(self) -> Moment<Utc> {
        Moment {
            time_zone: Utc,
            date_time: self.date_time,
            locale: LOCALE_EN_US.clone(),
        }
    }
}

impl Moment<FixedOffset> {
    pub fn new<T: Into<String>>(date: T, format: Option<T>) -> Result<Moment<FixedOffset>, String> {
        let date_string = date.into();

        if format.is_none() {
            if let Ok(date_time) = DateTime::parse_from_rfc3339(&date_string) {
                return Ok(Moment {
                    time_zone: date_time.timezone(),
                    date_time,
                    locale: LOCALE_EN_US.clone(),
                });
            }

            if let Ok(date_time) = DateTime::parse_from_rfc2822(&date_string) {
                return Ok(Moment {
                    time_zone: date_time.timezone(),
                    date_time,
                    locale: LOCALE_EN_US.clone(),
                });
            }

            return Err(format!(
                "Could not parse date, \"{}\", as RFC 3339 / ISO 8601 or RFC 2822.",
                date_string
            ));
        }

        let format_string = format.unwrap().into();

        if let Ok(date_time) = DateTime::parse_from_str(&date_string, &format_string) {
            return Ok(Moment {
                time_zone: date_time.timezone(),
                date_time,
                locale: LOCALE_EN_US.clone(),
            });
        }

        Err(format!(
            "Date, \"{}\", could not be parsed with format string \"{}\"",
            date_string, format_string
        ))
    }

    pub fn utc(self) -> Moment<Utc> {
        Moment {
            time_zone: Utc,
            date_time: self.date_time,
            locale: LOCALE_EN_US.clone(),
        }
    }
}

impl Moment<Utc> {
    pub fn utc<T: Into<String>>(date: Option<T>, format: Option<T>) -> Result<Moment<Utc>, String> {
        if date.is_none() {
            return Ok(Moment {
                time_zone: Utc,
                date_time: Utc::now().with_timezone(&Utc.fix()),
                locale: LOCALE_EN_US.clone(),
            });
        }

        Ok(Moment::<FixedOffset>::new(date.unwrap(), format)?.utc())
    }
}

impl<T: TimeZone + Debug> Moment<T> {
    pub fn locale(self, locale: Locale) -> Moment<T> {
        let mut moment = self.clone();
        moment.locale = locale;
        moment
    }

    fn add_subtract(self, duration: Duration, is_adding: bool) -> Moment<T> {
        let mut moment = self.clone();
        moment.date_time = if is_adding {
            self.checked_add_signed(duration).unwrap()
        } else {
            self.checked_sub_signed(duration).unwrap()
        };
        moment
    }

    // We want this to emultate Moment.js's API
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, duration: Duration) -> Moment<T> {
        self.add_subtract(duration, true)
    }

    pub fn subtract(self, duration: Duration) -> Moment<T> {
        self.add_subtract(duration, false)
    }

    pub fn start_of(self, unit: UnitOfTime) -> Moment<T> {
        let mut moment = self.clone();
        moment.date_time = match unit {
            UnitOfTime::Nanosecond => self.date_time,
            UnitOfTime::Microsecond => self.with_nanosecond(0).unwrap(),
            UnitOfTime::Millisecond => self
                .start_of(UnitOfTime::Microsecond)
                .with_nanosecond(1000)
                .unwrap(),
            UnitOfTime::Second => self
                .start_of(UnitOfTime::Millisecond)
                .with_nanosecond(1_000_000)
                .unwrap(),
            UnitOfTime::Minute => self.start_of(UnitOfTime::Second).with_second(0).unwrap(),
            UnitOfTime::Hour => self.start_of(UnitOfTime::Minute).with_minute(0).unwrap(),
            UnitOfTime::Day => self.start_of(UnitOfTime::Hour).with_hour(0).unwrap(),
            UnitOfTime::Week => {
                let new_day = self.day0() - self.locale_aware_day_of_week();
                self.start_of(UnitOfTime::Day).with_day0(new_day).unwrap()
            }
            UnitOfTime::IsoWeek => {
                let new_day = self.day0() - self.weekday().num_days_from_monday();
                self.start_of(UnitOfTime::Day).with_day0(new_day).unwrap()
            }
            UnitOfTime::Quarter => {
                let new_month = ((self.month() as f32 / 3_f32).floor() * 3_f32) as u32;
                self.start_of(UnitOfTime::Month)
                    .with_month(new_month)
                    .unwrap()
            }
            UnitOfTime::Month => self.start_of(UnitOfTime::Day).with_day0(0).unwrap(),
            UnitOfTime::Year => self.start_of(UnitOfTime::Month).with_month0(0).unwrap(),
        };
        moment
    }

    pub fn end_of(self, unit: UnitOfTime) -> Moment<T> {
        let mut durations = HashSet::new();
        durations.insert((1, UnitOfTime::Nanosecond));
        self.start_of(unit)
            .add(duration(durations))
            .subtract(Duration::nanoseconds(1))
    }

    pub(crate) fn locale_aware_day_of_week(&self) -> u32 {
        ((self.weekday().num_days_from_sunday() as i32 - self.locale.week.dow as i32 % 7 + 7) % 7)
            as u32
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    fn days_in_year(&self, year: i32) -> u32 {
        if Moment::<T>::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn first_week_offset(&self, year: i32) -> u32 {
        let first_week_day = 7_i32 + self.locale.week.dow as i32 - self.locale.week.doy as i32;
        let first_week_day_local_weekday = (7_i32
            + Utc
                .ymd(year, 0, first_week_day as u32)
                .weekday()
                .num_days_from_sunday() as i32
            - self.locale.week.dow as i32)
            % 7_i32;
        (-first_week_day_local_weekday + first_week_day - 1) as u32
    }

    fn weeks_in_year(&self, year: i32) -> u32 {
        let week_offset = self.first_week_offset(year);
        let week_offset_next = self.first_week_offset(year + 1);
        ((self.days_in_year(year) as i32 - week_offset as i32 + week_offset_next as i32) / 7_i32)
            as u32
    }

    pub(crate) fn locale_aware_week_of_year(&self) -> (u32, i32) {
        let week_offset = self.first_week_offset(self.year());
        let week =
            ((self.ordinal() as f32 - week_offset as f32 - 1_f32) / 7_f32).floor() as u32 + 1;

        if week < 1 {
            let year = self.year() - 1;
            return (week + self.weeks_in_year(year), year);
        } else if week > self.weeks_in_year(self.year()) {
            return (week - self.weeks_in_year(self.year()), self.year() + 1);
        }

        (week, self.year())
    }

    pub fn format(&self, format: String) -> String {
        self.locale.format(&self, format)
    }

    pub fn calendar(
        &self,
        reference_moment: Moment<T>,
        formats: Option<&Calendar>,
        calendar_format: Option<CalendarFormatFn<T>>,
    ) -> String {
        self.locale.calendar(&self, reference_moment, formats, calendar_format)
    }
}
