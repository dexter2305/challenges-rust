use std::{collections::HashMap, ops::Deref};

/// 383. Ransom note
///
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine_dict: HashMap<char, i32> = HashMap::new();
    for ch in magazine.chars() {
        magazine_dict
            .entry(ch)
            .and_modify(|c| *c = *c + 1)
            .or_insert(1);
    }

    for ch in ransom_note.chars() {
        let maybe_entry = magazine_dict.get(&ch);
        let x = match maybe_entry {
            None => return false,
            Some(counter) if *counter == 0 => return false,
            Some(counter) => magazine_dict.entry(ch).and_modify(|c| *c = *c -1)
        };
    }
    return true
}

#[cfg(test)]
mod test {

    use crate::leetcode::p0383;

    #[test]
    fn can_construct() {
        let tests = vec![("aa", "bb", false), ("aa", "ab", false), ("aa", "aab", true)];

        for (ransome_note, magazine, exp) in tests {
            assert_eq!(
                p0383::can_construct(String::from(ransome_note), String::from(magazine)),
                exp,
                "Given ransome_note = {} and magazine = {}, expected {}",
                ransome_note,
                magazine,
                exp
            )
        }
    }
}
