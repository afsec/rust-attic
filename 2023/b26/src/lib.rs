#[cfg(test)]
mod tests;

use anyhow::{bail, format_err};
use bytes::Bytes;
type AppResult<T> = anyhow::Result<T>;

const BASE26: &str = "abcdefghijklmnopqrstuvwxyz";
const WORD_MAX_LENGTH: u8 = 14;

#[derive(Debug, Default)]
pub struct Base26 {
    strict_mode: bool,
}

impl Base26 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_strict_mode(&mut self) {
        self.strict_mode = true;
    }
    pub fn encode(&self, word: &str, length: u8) -> AppResult<u64> {
        if length > WORD_MAX_LENGTH {
            bail!("Strict mode enabled: A word must at maximum {WORD_MAX_LENGTH} characters.");
        }

        if word.chars().fold(0, |acc, _| acc + 1) > length {
            bail!("A word cannot be greater than {length}");
        }

        word.chars().try_for_each(|c| {
            let has_valid_chars = if self.strict_mode {
                c.is_ascii_lowercase()
            } else {
                c.is_ascii_alphabetic()
            };

            if has_valid_chars {
                Ok(())
            } else {
                bail!("A word must have ascii characters only: [{BASE26}]");
            }
        })?;

        let bytes = word
            .to_ascii_lowercase()
            .chars()
            .map(|c| Self::get_idx(c))
            .collect::<AppResult<Bytes>>()?;

        let mut coeficient = length;

        let encoded = bytes
            .iter()
            .map(move |x| {
                coeficient -= 1;
                u64::from(*x) * 26u64.pow(u32::from(coeficient))
            })
            .fold(0u64, |acc, item| acc + u64::from(item));

        Ok(encoded)
    }

    pub fn decode(&self, encoded: u64, length: u8) -> AppResult<String> {
        let mut coeficient = length.into();
        let mut number = encoded;
        let decoded = (0..length)
            .into_iter()
            .map(move |_| {
                coeficient -= 1;
                let cur_power = 26u64.pow(coeficient);
                let factor = number / cur_power;
                number -= factor * cur_power;
                let char_idx = usize::try_from(factor)?;
                let retrieved_char = BASE26.chars().nth(char_idx).ok_or(format_err!(
                    "Wrong length [{length}] for decoding [{encoded}]"
                ))?;
                Ok(retrieved_char)
            })
            .collect::<AppResult<String>>()?;

        Ok(decoded)
    }

    fn get_idx(input_c: char) -> AppResult<u8> {
        let outcome = BASE26
            .chars()
            .enumerate()
            .filter(|(_, c)| input_c == *c)
            .collect::<Vec<(usize, char)>>();

        let (idx, _) = outcome.first().ok_or(format_err!("Impossible state"))?;
        Ok(u8::try_from(*idx)?)
    }
}
