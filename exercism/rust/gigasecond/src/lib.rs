use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    DateTime::checked_add(start, Duration::seconds(1_000_000_000)).unwrap()
}
