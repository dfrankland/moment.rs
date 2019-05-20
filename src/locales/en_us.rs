use crate::locale::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref LOCALE_EN_US: Locale = Locale {
        invalid_date: "Invalid Date",
        months: MonthStrings(
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December"
        ),
        months_short: MonthStrings(
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
        ),
        weekdays: WeekDayStrings(
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday"
        ),
        weekdays_short: WeekDayStrings("Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"),
        weekdays_min: WeekDayStrings("Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"),
        long_date_format: LongDateFormat {
            LT: "h:mm A",
            LTS: "h:mm:ss A",
            L: "MM/DD/YYYY",
            LL: "MMMM D, YYYY",
            LLL: "MMMM D, YYYY h:mm A",
            LLLL: "dddd, MMMM D, YYYY h:mm A"
        },
        calendar: Calendar {
            same_day: "[Today at] LT",
            next_day: "[Tomorrow at] LT",
            next_week: "dddd [at] LT",
            last_day: "[Yesterday at] LT",
            last_week: "[Last] dddd [at] LT",
            same_else: "L"
        },
        relative_time: RelativeTime {
            future: "in %s",
            past: "%s ago",
            s: "a few seconds",
            ss: "%d seconds",
            m: "a minute",
            mm: "%d minutes",
            h: "an hour",
            hh: "%d hours",
            d: "a day",
            dd: "%d days",
            M: "a month",
            MM: "%d months",
            y: "a year",
            yy: "%d years"
        },
        day_of_month_ordinal_parse: Regex::new(r"\d{1,2}(st|nd|rd|th)").unwrap(),
        ordinal: |number| {
            let b = number % 10;
            let output = if !!(number % 100 / 10) == 1 {
                "th"
            } else {
                match b {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                }
            };

            format!("{}{}", number, output)
        },
        week: Week { dow: 0, doy: 6 },
        meridiem: |hour, _| {
            if hour < 12 {
                String::from("AM")
            } else {
                String::from("PM")
            }
        },
    };
}
