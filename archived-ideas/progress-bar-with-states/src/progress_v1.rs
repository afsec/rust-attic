const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub fn progress<Iter>(iter: Iter, f: fn(Iter::Item) -> ())
where
    Iter: Iterator,
{
    let mut i: usize = 1;
    for n in iter {
        println!("{} {}", CLEAR, "*".repeat(i));
        i += 1;
        f(n);
    }
}

// pub fn progress<Iter, T>(iter: Iter, f: fn(T) -> ())
// where
//     Iter: Iterator<Item = T>,
// {
//     let mut i: usize = 1;
//     for n in iter {
//         println!("{} {}", CLEAR, "*".repeat(i));
//         i += 1;
//         f(n);
//     }
// }
