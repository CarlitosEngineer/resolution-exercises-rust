use time::{Duration, PrimitiveDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    const GIGASECOND: i64 = 1_000_000_000;
    start + Duration::seconds(GIGASECOND)
}