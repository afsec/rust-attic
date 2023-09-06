const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self { iter, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{} {}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}
