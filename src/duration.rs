use crate::UnitOfTime;
use chrono::Duration;
use std::collections::HashSet;

pub fn duration<T>(durations: HashSet<(i64, UnitOfTime), T>) -> Duration {
    let mut total_duration = Duration::zero();
    for (value, unit) in durations.iter() {
        let value = *value;
        let duration_to_add = match unit {
            UnitOfTime::Nanosecond => Duration::nanoseconds(value),
            UnitOfTime::Microsecond => Duration::microseconds(value),
            UnitOfTime::Millisecond => Duration::milliseconds(value),
            UnitOfTime::Second => Duration::seconds(value),
            UnitOfTime::Minute => Duration::minutes(value),
            UnitOfTime::Hour => Duration::hours(value),
            UnitOfTime::Day => Duration::days(value),
            UnitOfTime::Week | UnitOfTime::IsoWeek => Duration::weeks(value),
            UnitOfTime::Quarter => {
                let mut durations = HashSet::new();
                durations.insert((value * 3, UnitOfTime::Month));
                duration(durations)
            }
            UnitOfTime::Month => Duration::weeks(value * 4),
            UnitOfTime::Year => {
                let mut durations = HashSet::new();
                durations.insert((value * 12, UnitOfTime::Month));
                duration(durations)
            }
        };
        total_duration = total_duration.checked_add(&duration_to_add).unwrap();
    }
    total_duration
}
