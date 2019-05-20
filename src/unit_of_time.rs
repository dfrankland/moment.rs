#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum UnitOfTime {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    IsoWeek,
    Week,
    Quarter,
    Month,
    Year,
}
