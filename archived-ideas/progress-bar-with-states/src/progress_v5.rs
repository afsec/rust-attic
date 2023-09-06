const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// * States

pub(in crate) struct Unbounded;

pub(in crate) struct Bounded {
    bound: usize,
    delims: (char, char),
}

pub(in crate) struct Progress<ITER, STATE> {
    iter: ITER,
    i: usize,
    state: STATE,
}

pub(in crate) trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1
        );
    }
}

pub(in crate) trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<ITER> ProgressIteratorExt for ITER
where
    ITER: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

impl<ITER> Progress<ITER, Unbounded> {
    pub fn new(iter: ITER) -> Self {
        Self {
            iter,
            i: 0,
            state: Unbounded,
        }
    }
}

impl<ITER> Progress<ITER, Unbounded>
where
    ITER: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<ITER, Bounded> {
        let bounded = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            i: self.i,
            iter: self.iter,
            state: bounded,
        }
    }
}

impl<ITER, STATE> Iterator for Progress<ITER, STATE>
where
    ITER: Iterator,
    STATE: ProgressDisplay,
{
    type Item = ITER::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        self.state.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

impl<ITER> Progress<ITER, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.state.delims = delims;
        self
    }
}
