use b26::Base26;
const MAX_BLOCK_HEIGHT: u64 = 2_099_999_997_689_999;

use self::sat::Sat;

struct WordToCheck {
    word: &'static str,
    number: u64,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use_case_1()?;
    use_case_2()?;
    Ok(())
}

fn use_case_1() -> Result<(), Box<dyn std::error::Error>> {
    let ord1 = WordToCheck {
        word: "nvtdijuwxlp",
        number: 0,
    };
    let sat = Sat::from_name(ord1.word)?;

    dbg!(sat.n(), sat.name());

    let word = ord1.word;
    let number = ord1.number;

    assert_eq!(sat.n(), number);
    assert_eq!(sat.name(), word);

    let sat_b26 = Sat::from(0);

    assert_eq!(sat_b26.n(), number);
    assert_eq!(sat_b26.name(), word);
    Ok(())
}

fn use_case_2() -> Result<(), Box<dyn std::error::Error>> {
    let ord1 = WordToCheck {
        word: "a",
        number: MAX_BLOCK_HEIGHT,
    };

    let sat = Sat::from_name(ord1.word)?;

    dbg!(sat.n(), sat.name());

    let word = ord1.word;
    let number = ord1.number;

    assert_eq!(sat.n(), number);
    assert_eq!(sat.name(), word);

    Ok(())
}

mod sat {
    use anyhow::bail;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
    pub struct Sat(u64);

    impl From<u64> for Sat {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl Sat {
        pub(crate) const SUPPLY: u64 = 2_099_999_997_690_000;

        pub(crate) fn n(self) -> u64 {
            self.0
        }

        pub(crate) fn name(self) -> String {
            let mut x = Self::SUPPLY - self.0;
            let mut name = String::new();
            while x > 0 {
                name.push(
                    "abcdefghijklmnopqrstuvwxyz"
                        .chars()
                        .nth(((x - 1) % 26) as usize)
                        .unwrap(),
                );
                x = (x - 1) / 26;
            }
            name.chars().rev().collect()
        }

        pub fn from_name(name: &str) -> anyhow::Result<Self> {
            let mut x = 0;
            name.chars().try_for_each(|c| {
                match c {
                    'a'..='z' => {
                        x = x * 26 + c as u64 - 'a' as u64 + 1;
                    }
                    _ => bail!("invalid character in sat name: {c}"),
                }
                Ok(())
            })?;
            if x > Self::SUPPLY {
                bail!("sat name out of range");
            }
            Ok(Sat(Self::SUPPLY - x))
        }
    }
}
