use crate::{AppResult, Base26};
use anyhow::Ok;
use test_case::test_case;

#[derive(Debug, PartialEq, Eq)]
enum ExpectedResult {
    Success,
    Fail,
}
impl<T, E> From<&Result<T, E>> for ExpectedResult {
    fn from(res: &Result<T, E>) -> Self {
        match res {
            Result::Ok(_) => Self::Success,
            Result::Err(_) => Self::Fail,
        }
    }
}

#[derive(Debug)]
struct WordToTest {
    word: &'static str,
    number: u64,
}
const WORDS_TO_TEST: [WordToTest; 8] = [
    WordToTest {
        word: "CaT",
        number: 1371,
    },
    WordToTest {
        word: "LEAF",
        number: 196045,
    },
    WordToTest {
        word: "BYE",
        number: 1304,
    },
    WordToTest {
        word: "DOg",
        number: 2398,
    },
    WordToTest {
        word: "aCE",
        number: 56,
    },
    WordToTest {
        word: "HeLP",
        number: 126037,
    },
    WordToTest {
        word: "FAt",
        number: 3399,
    },
    WordToTest {
        word: "PARAlLELiSM",
        number: 2121060025694232,
    },
];

const WRONG_WORDS_TO_TEST: [WordToTest; 5] = [
    WordToTest {
        word: "FAT",
        number: 1371,
    },
    WordToTest {
        word: "DOT",
        number: 2398,
    },
    WordToTest {
        word: "ACE",
        number: 516,
    },
    WordToTest {
        word: "HELP",
        number: 126137,
    },
    WordToTest {
        word: "PARALELISM",
        number: 2121060025694232,
    },
];

#[test_case(&WORDS_TO_TEST, true, ExpectedResult::Fail ; "when strict_mode is enabled")]
#[test_case(&WORDS_TO_TEST, false, ExpectedResult::Success ; "when strict_mode is disabled")]
fn encode(
    words_to_test: &[WordToTest],
    strict_mode: bool,
    expected: ExpectedResult,
) -> AppResult<()> {
    let mut base26 = Base26::new();

    if strict_mode {
        base26.set_strict_mode();
    }

    for item in words_to_test {
        let word = item.word;
        let number = item.number;
        let encoded = base26.encode(word, word.len().try_into()?);

        assert_eq!(ExpectedResult::from(&encoded), expected);

        if expected == ExpectedResult::Success {
            assert_eq!(encoded?, number);
        }
    }

    Ok(())
}

#[test_case(&WRONG_WORDS_TO_TEST, ExpectedResult::Fail ; "when input is wrong")]
#[test_case(&WORDS_TO_TEST, ExpectedResult::Success ; "when both input and length are right")]
fn decode(words_to_test: &[WordToTest], expected: ExpectedResult) -> AppResult<()> {
    let base26 = Base26::new();

    for item in words_to_test {
        let word = item.word.to_lowercase();
        let number = item.number;
        let decoded = base26.decode(number, word.len().try_into()?);

        dbg!(&item, &decoded);

        if expected == ExpectedResult::Success {
            assert_eq!(decoded?, word);
        } else {
            if decoded.is_ok() {
                assert!(decoded? != word);
            }
        }
    }

    Ok(())
}

#[test_case(&WORDS_TO_TEST; "when input is valid")]
fn encode_and_decode(words_to_test: &[WordToTest]) -> AppResult<()> {
    let base26 = Base26::new();

    for item in words_to_test {
        let word = item.word;
        let number = item.number;
        let encoded = base26.encode(word, word.len().try_into()?);

        assert!(encoded.is_ok());

        let encoded = encoded?;

        assert_eq!(encoded, number);

        let decoded = base26.decode(encoded, word.len().try_into()?);

        assert_eq!(word.to_ascii_lowercase(), decoded?.as_str())
    }

    Ok(())
}
