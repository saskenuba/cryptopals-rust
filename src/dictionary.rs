use std::collections::BTreeMap;

static ENGLISH_LETTERS: &[char] = &['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', ' ', '\"'];

/// The higher, the better
pub fn score_english_letters_phrase<T>(s: T) -> i32
where
    T: AsRef<str>,
{
    let original_str = s.as_ref();
    let mut set = BTreeMap::new();

    let original_str = original_str.to_ascii_lowercase();
    let chars = original_str.chars();

    chars.filter(|c| !char::is_whitespace(*c)).for_each(|char| {
        let score = if char.is_alphabetic() { 1 } else { -3 };
        set.entry(char)
            .and_modify(|count| *count += score)
            .or_insert(score);
    });
    set.into_values().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_detect() {
        let phrase = "The quick brown fox jumps over the lazy dog? That's fake!";
        let freq = score_english_letters_phrase(phrase);
        assert_eq!(freq, 3);
    }
}
