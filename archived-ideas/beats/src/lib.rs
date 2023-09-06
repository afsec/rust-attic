#![warn(clippy::all)]
// source: https://en.wikipedia.org/wiki/Swatch_Internet_Time

// ## Truths
// Unit Beat Conversion
// 1 day    1000 .beats
// 1 hour   41.6 .beats
// 1 min    0.694 .beats
// 1 s      0.011574 .beats

fn get_num_seconds_from_midnight() -> u32 {
    use chrono::prelude::*;
    let bmt = FixedOffset::east(60 * 60);
    let now: DateTime<FixedOffset> = Utc::now().with_timezone(&bmt);
    now.num_seconds_from_midnight()
}

pub fn get_current_beats() -> u16 {
    ((get_num_seconds_from_midnight() as f32) * (0.011_574 as f32)) as u16
}

