#![warn(clippy::all)]

// ## Truths
// Unit Beat Conversion
// 1 day    1000 .beats
// 1 hour   41.6 .beats
// 1 min    0.694 .beats
// 1 s      0.011574 .beats

fn get_current_time() -> String {
    use chrono::prelude::*;
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}

fn get_current_beat() -> String {
    // Current time
    let current_time = get_current_time();
    // Parse Hour, Minute and Second
    let time: Vec<&str> = current_time.split(':').collect();
    let hours: u32 = time[0].parse().unwrap();
    let minutes: u32 = time[1].parse().unwrap();
    let seconds: u32 = time[2].parse().unwrap();
    // Convert Hour and Minute in seconds
    let secs_of_day: u32 = (seconds) + (minutes * 60) + (hours * 60 * 60);
    let beats = ((secs_of_day as f32) * (0.011_574 as f32)) as u32;
    // let beats = (secs_of_day as f32) * (0.011_574 as f32);
    beats.to_string()

    //   println!("Local: {}:{}", hours, minutes);
    //   println!("Local: {}", current_time);
    //   let msg = r#"
    //   "Internet Time is not geopolitical, it is global."
    //                               - NICHOLAS NEGROPONTE
    //   "#;
    //   println!("{}", msg);
}

fn main() {
    let beats = get_current_beat();
    println!("@{:03}", beats);
}
