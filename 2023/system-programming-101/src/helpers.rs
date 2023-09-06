use std::fmt::Debug;

pub(crate) trait CalcAlocated {
    // fn consumed_bytes(&self) -> usize;
    fn consumed_bytes(&self) -> usize where Self: Sized {
        use std::mem;
        mem::size_of::<Self>()
    }
    fn memory_consumption_report(&self) where Self: Sized + Debug {
        println!("Memory Consumption Report");
        println!("=========================");
        println!("");
        println!("{:?}: {} Bytes",self ,self.consumed_bytes());
    }
}
