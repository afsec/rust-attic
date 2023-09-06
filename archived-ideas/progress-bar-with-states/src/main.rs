use std::collections::HashSet;

mod progress_v1;
mod progress_v2;
mod progress_v3;
mod progress_v4;
mod progress_v5;

fn expensive_calculation<T>(_n: T) {
    use std::{thread::sleep, time::Duration};
    println!("Processing...");
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1u8, 2, 3];

    let h = HashSet::from([1, 2, 3]);
    // {
    //     dbg!(&v);
    //     progress_simple::progress(v.iter(), expensive_calculation);
    //     dbg!(&h);
    //     progress_simple::progress(h.iter(), expensive_calculation);
    // }
    // {
    //     dbg!(&v);
    //     std::thread::sleep(std::time::Duration::from_secs(3));
    //     for n in progress_middle::Progress::new(v.iter()) {
    //         expensive_calculation(n);
    //     }
    //     dbg!(&h);
    //     std::thread::sleep(std::time::Duration::from_secs(3));
    //     for n in progress_middle::Progress::new(h.iter()) {
    //         expensive_calculation(n);
    //     }
    // }
    // {
    //     use progress_v3::ProgressIteratorExt;
    //     dbg!(&v);
    //     std::thread::sleep(std::time::Duration::from_secs(3));
    //     for n in v.iter().progress() {
    //         expensive_calculation(n);
    //     }
    //     dbg!(&h);
    //     std::thread::sleep(std::time::Duration::from_secs(3));
    //     for n in h.iter().progress() {
    //         expensive_calculation(n);
    //     }
    // }
    // {
    //     use progress_v4::ProgressIteratorExt;
    //     dbg!(&v);
    //     std::thread::sleep(std::time::Duration::from_secs(3));
    //     for n in v.iter().progress().with_bound() {
    //         expensive_calculation(n);
    //     }
    //     dbg!(&h);
    //     std::thread::sleep(std::time::Duration::from_secs(3));
    //     for n in (0..).progress() {
    //         expensive_calculation(n);
    //     }
    // }
    {
        use progress_v5::ProgressIteratorExt;
        dbg!(&v);
        let delims = ('<', '>');
        std::thread::sleep(std::time::Duration::from_secs(3));
        for n in v.iter().progress().with_bound().with_delims(delims) {
            expensive_calculation(n);
        }
        dbg!(&h);
        std::thread::sleep(std::time::Duration::from_secs(3));
        for n in (0..).progress() {
            // for n in (0..).progress().with_delims(delims) {
            expensive_calculation(n);
        }
    }
}
