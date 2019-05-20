use chrono::prelude::*;
use moment::{duration, Moment, UnitOfTime};
use std::collections::HashSet;

// TODO: Make test that compares output to moment.js

#[test]
fn test() {
    let moment = Moment::<Local>::new().unwrap();

    // TODO: Make assertion on format
    let ymd = moment.format(String::from(r"YYYY-MM-DD [escaped]\!"));
    // assert_eq!(ymd, String::from("2019-05-20 escaped!"));
    println!("{}", ymd);

    // TODO: Make assertion on calendar
    let mut durations = HashSet::new();
    durations.insert((5, UnitOfTime::Day));
    let cal = moment.calendar(moment.clone().subtract(duration(durations)), None, None);
    // assert_eq!(cal, String::from("Monday at 12:48 AM"));
    println!("{}", cal);
}
