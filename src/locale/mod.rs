mod calendar_format;
mod calendar_strings;
mod long_date_format_strings;
mod month_strings;
mod relative_time_strings;
mod week_config;
mod weekday_strings;

pub use self::{
    calendar_format::*, calendar_strings::*, long_date_format_strings::*, month_strings::*,
    relative_time_strings::*, week_config::*, weekday_strings::*,
};
use crate::{Moment, UnitOfTime};
use chrono::prelude::*;
use lazy_static::lazy_static;
use num_integer::div_mod_floor;
use regex::{Captures, Regex};
use std::fmt::Debug;

lazy_static! {
    static ref FORMATTING_TOKENS: Regex = Regex::new(r"(\[[^\[]*\])|(\\)?([Hh]mm(ss)?|Mo|MM?M?M?|Do|DDDo|DD?D?D?|ddd?d?|do?|w[o|w]?|W[o|W]?|Qo?|YYYYYY|YYYYY|YYYY|YY|gg(ggg?)?|GG(GGG?)?|e|E|a|A|hh?|HH?|kk?|mm?|ss?|S{1,9}|x|X|zz?|ZZ?|.)").unwrap();
    static ref LOCALE_FORMATTING_TOKENS: Regex = Regex::new(r"(\[[^\[]*\])|(\\)?(LTS|LT|LL?L?L?|l{1,4})").unwrap();
    static ref ESCAPED_TEXT: Regex = Regex::new(r"\[[\s\S]").unwrap();
    static ref ESCAPED_TEXT_DELIMETERS: Regex = Regex::new(r"^\[|\]$").unwrap();
}

#[derive(Debug, Clone)]
pub struct Locale {
    pub invalid_date: &'static str,
    pub months: MonthStrings,
    pub months_short: MonthStrings,
    pub weekdays: WeekDayStrings,
    pub weekdays_short: WeekDayStrings,
    pub weekdays_min: WeekDayStrings,
    pub long_date_format: LongDateFormat,
    pub calendar: Calendar,
    pub relative_time: RelativeTime,
    pub day_of_month_ordinal_parse: Regex,
    pub ordinal: fn(i32) -> String,
    pub week: Week,
    pub meridiem: fn(u32, u32) -> String,
}

impl Locale {
    fn expand_format(&self, format: String) -> String {
        let mut expanded_format = format;
        let mut last_expanded_format = expanded_format.clone();
        loop {
            let new_expanded_format = {
                LOCALE_FORMATTING_TOKENS.replace_all(&expanded_format, |captures: &Captures| {
                    let mut replacement_value = String::from("");
                    if let Some(match_text) = captures.get(0) {
                        if let Some(value) =
                            self.long_date_format.get_by_key(match_text.as_str())
                        {
                            replacement_value = format!("{}{}", replacement_value, value);
                        } else {
                            replacement_value = format!("{}{}", replacement_value, match_text.as_str());
                        }
                    }
                    replacement_value
                })
            };
            expanded_format = String::from(new_expanded_format);
            if expanded_format == last_expanded_format {
                return expanded_format;
            }
            last_expanded_format = expanded_format.clone();
        }
    }

    fn remove_formatting_tokens(&self, input: &str) -> String {
        if ESCAPED_TEXT.is_match(input) {
            return String::from(ESCAPED_TEXT_DELIMETERS.replace_all(input, ""));
        }
        input.replace(r"\", "")
    }

    fn format_tokens<T: TimeZone + Debug>(&self, token: &str, moment: &Moment<T>) -> Option<String> {
        match token {
            // Month
            "M" => Some(format!("{}", moment.month())),
            "Mo" => Some((self.ordinal)(moment.month() as i32)),
            "MM" => Some(format!("{:02}", moment.month())),
            "MMM" => Some(String::from(
                self.months_short.get_index(moment.month0()).unwrap(),
            )),
            "MMMM" => Some(String::from(
                self.months.get_index(moment.month0()).unwrap(),
            )),

            // Quarter
            "Q" => Some(format!("{}", (moment.month() as f32 / 3_f32).ceil() as u32)),
            "Qo" => Some((self.ordinal)((moment.month() as f32 / 3_f32).ceil() as i32)),

            // Day of Month
            "D" => Some(format!("{}", moment.day())),
            "Do" => Some((self.ordinal)(moment.day() as i32)),
            "DD" => Some(format!("{:02}", moment.day())),

            // Day of Year
            "DDD" => Some(format!("{}", moment.ordinal())),
            "DDDo" => Some((self.ordinal)(moment.ordinal() as i32)),
            "DDDD" => Some(format!("{:03}", moment.ordinal())),

            // Day of Week
            "d" => Some(format!("{}", moment.locale_aware_day_of_week())),
            "do" => Some((self.ordinal)(moment.locale_aware_day_of_week() as i32)),
            "dd" => Some(String::from(
                self.weekdays_min
                    .get_by_index(moment.weekday().num_days_from_sunday())
                    .unwrap(),
            )),
            "ddd" => Some(String::from(
                self.weekdays_short
                    .get_by_index(moment.weekday().num_days_from_sunday())
                    .unwrap(),
            )),
            "dddd" => Some(String::from(
                self.weekdays
                    .get_by_index(moment.weekday().num_days_from_sunday())
                    .unwrap(),
            )),

            // Day of Week (Locale)
            "e" => Some(format!("{}", moment.locale_aware_day_of_week())),

            // Day of Week (ISO)
            "E" => Some(format!("{}", moment.weekday().num_days_from_monday())),

            // Week of Year
            "w" => Some(format!("{}", moment.iso_week().week())),
            "wo" => Some((self.ordinal)(moment.iso_week().week() as i32)),
            "ww" => Some(format!("{:02}", moment.iso_week().week())),

            // Year
            "YY" => Some(format!("{:02}", moment.year_ce().1 % 100)),
            "YYYY" => Some(format!("{:04}", moment.year_ce().1)),
            "Y" => {
                let (positive, year) = moment.year_ce();
                Some(format!("{:04}", year as i32 * positive as i32))
            }

            // Week Year
            "gg" => Some(format!("{:02}", moment.locale_aware_week_of_year().1 % 100)),
            "gggg" => Some(format!("{:04}", moment.locale_aware_week_of_year().1)),

            // Week Year (ISO)
            "GG" => Some(format!("{:02}", moment.iso_week().year() % 100)),
            "GGGG" => Some(format!("{:04}", moment.iso_week().year())),

            // AM/PM
            "A" => Some((self.meridiem)(moment.hour(), moment.minute())),
            "a" => Some((self.meridiem)(moment.hour(), moment.minute()).to_lowercase()),

            // Hour
            "H" => Some(format!("{}", moment.hour())),
            "HH" => Some(format!("{:02}", moment.hour())),
            "h" => Some(format!("{}", moment.hour12().1)),
            "hh" => Some(format!("{:02}", moment.hour12().1)),
            "k" => Some(format!("{}", moment.hour() + 1)),
            "kk" => Some(format!("{:02}", moment.hour() + 1)),

            // Minute
            "m" => Some(format!("{}", moment.minute())),
            "mm" => Some(format!("{:02}", moment.minute())),

            // Second
            "s" => Some(format!("{}", moment.second())),
            "ss" => Some(format!("{:02}", moment.second())),

            // Fractional Second
            "S" => Some(format!("{}", moment.nanosecond() / 100_000_000)),
            "SS" => Some(format!("{:02}", moment.nanosecond() / 10_000_000)),
            "SSS" => Some(format!("{:03}", moment.nanosecond() / 1_000_000)),
            "SSSS" => Some(format!("{:04}", moment.nanosecond() / 100_000)),
            "SSSSS" => Some(format!("{:05}", moment.nanosecond() / 10000)),
            "SSSSSS" => Some(format!("{:06}", moment.nanosecond() / 1000)),
            "SSSSSSS" => Some(format!("{:06}", moment.nanosecond() / 100)),
            "SSSSSSSS" => Some(format!("{:08}", moment.nanosecond() / 10)),
            "SSSSSSSSS" => Some(format!("{:09}", moment.nanosecond())),

            // Time Zone
            // TODO: Find a way to do timezones names.
            "z" => unimplemented!("Timezones are difficult..."),
            "Z" => {
                let offset = moment.offset().local_minus_utc();
                let (sign, offset) = if offset < 0 {
                    ('-', -offset)
                } else {
                    ('+', offset)
                };
                let (mins, ..) = div_mod_floor(offset, 60);
                let (hour, min) = div_mod_floor(mins, 60);
                Some(format!("{}{:02}:{:02}", sign, hour, min))
            }
            "ZZ" => {
                let offset = moment.offset().local_minus_utc();
                let (sign, offset) = if offset < 0 {
                    ('-', -offset)
                } else {
                    ('+', offset)
                };
                let (mins, ..) = div_mod_floor(offset, 60);
                let (hour, min) = div_mod_floor(mins, 60);
                Some(format!("{}{:02}{:02}", sign, hour, min))
            }

            // Unix Timestamp
            "X" => Some(format!("{}", moment.timestamp())),

            // Unix Millisecond Timestamp
            "x" => Some(format!("{}", moment.timestamp_millis())),

            _ => None,
        }
    }

    pub fn format<T: TimeZone + Debug>(&self, moment: &Moment<T>, format: String) -> String {
        let expanded_format = self.expand_format(format);

        let formatted_string =
            FORMATTING_TOKENS.replace_all(&expanded_format, |captures: &Captures| {
                let mut replacement_value = String::from("");
                if let Some(match_text) = captures.get(0) {
                    let input = match_text.as_str();
                    let value = self
                        .format_tokens(input, &moment)
                        .unwrap_or_else(|| self.remove_formatting_tokens(input));
                    replacement_value = format!("{}{}", replacement_value, value);
                }
                replacement_value
            });

        String::from(formatted_string)
    }

    pub fn calendar<T: TimeZone + Debug>(
        &self,
        moment: &Moment<T>,
        reference_moment: Moment<T>,
        formats: Option<&Calendar>,
        calendar_format: Option<CalendarFormatFn<T>>,
    ) -> String {
        let calendar_format_fn = calendar_format.unwrap_or(default_calendar_format);
        let calendar_format =
            calendar_format_fn(moment, &reference_moment.start_of(UnitOfTime::Day));
        let format = formats
            .unwrap_or(&self.calendar)
            .get_format_for_calendar_format(calendar_format);
        self.format(moment, String::from(format))
    }
}

pub type CalendarFormatFn<T> = fn(&Moment<T>, &Moment<T>) -> CalendarFormat;

pub fn default_calendar_format<T: TimeZone + Debug>(
    moment: &Moment<T>,
    reference_moment: &Moment<T>,
) -> CalendarFormat {
    let diff = moment.signed_duration_since(**reference_moment).num_days();
    if diff < -6 {
        return CalendarFormat::SameElse;
    }
    if diff < -1 {
        return CalendarFormat::LastWeek;
    }
    if diff < 0 {
        return CalendarFormat::LastDay;
    }
    if diff < 1 {
        return CalendarFormat::SameDay;
    }
    if diff < 2 {
        return CalendarFormat::NextDay;
    }
    if diff < 7 {
        return CalendarFormat::NextWeek;
    }
    CalendarFormat::SameElse
}
