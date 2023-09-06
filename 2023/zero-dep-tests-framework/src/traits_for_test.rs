#[cfg(test)]
pub trait MockupData {
    fn mock_one(self) -> Self;
    fn mock_vec(self, items: usize) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum ProfileData {
    Development,
    DevelopmentTests,
    ReleaseTests,
    Release,
}