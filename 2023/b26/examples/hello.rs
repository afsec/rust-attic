use b26::Base26;
// const MAX_BLOCK_HEIGHT: u64 = 2_099_999_997_689_999;

// struct WordToCheck {
//     word: &'static str,
//     number: u64,
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use_case_1()?;
    use_case_2()?;
    Ok(())
}

fn use_case_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut base26 = Base26::new();
    base26.set_strict_mode();

    let word = "hello";

    let encoded = base26.encode(&word, word.len().try_into()?)?;

    let decoded = base26.decode(encoded, word.len().try_into()?)?;

    // dbg!(word, encoded, &decoded);

    assert_eq!(word, decoded.as_str());

    Ok(())
}

fn use_case_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut base26 = Base26::new();
    base26.set_strict_mode();
    let number = 1689265369;
    let decoded = base26.decode(number, 4)?;

    dbg!(number, &decoded);

    Ok(())
}
