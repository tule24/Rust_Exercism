use time::PrimitiveDateTime as DateTime;
use time_macros::{date, datetime, time};
use time::Duration;
use time::ext::NumericalDuration;

pub fn gigasecond(start: DateTime) -> DateTime{
    let dur = Duration::seconds(1_000_000_000);
    start.checked_add(dur).unwrap()

    // start + 1e9.seconds()
}

