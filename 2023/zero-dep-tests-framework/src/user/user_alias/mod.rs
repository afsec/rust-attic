use crate::traits_for_test::ProfileData;

#[derive(Debug)]
pub struct UserAlias(ProfileData);

// Tests:
// Works on: `cargo test`
//
#[cfg(all(debug_assertions, test))]
impl Default for UserAlias {
    fn default() -> Self {
        Self(ProfileData::DevelopmentTests)
    }
}

// Development:
// Works on: `cargo run`
//           `cargo build`
//
#[cfg(all(debug_assertions, not(test)))]
impl Default for UserAlias {
    fn default() -> Self {
        Self(ProfileData::Development)
    }
}

// Specific Tests on release
// Works on: `cargo test --release`
//
#[cfg(all(not(debug_assertions), test))]
impl Default for UserAlias {
    fn default() -> Self {
        Self(ProfileData::ReleaseTests)
    }
}

// Production:
// Works on: `cargo run --release`
//           `cargo build --release`
//
#[cfg(all(not(debug_assertions), not(test)))]
impl Default for UserAlias {
    fn default() -> Self {
        Self(ProfileData::Release)
    }
}
