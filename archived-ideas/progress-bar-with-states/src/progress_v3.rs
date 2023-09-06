const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub struct Progress<ITER> {
    iter: ITER,
    i: usize,
}

impl<ITER> Progress<ITER> {
    pub fn new(iter: ITER) -> Self {
        Self { iter, i: 0 }
    }
}

impl<ITER> Iterator for Progress<ITER>
where
    ITER: Iterator,
{
    type Item = ITER::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{} {}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

pub(crate) trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<ITER> ProgressIteratorExt for ITER
where
    ITER: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}
