fn is_palindrome_recursive<W: AsRef<str>>(sequence: W) -> bool {
    let sequence: Vec<char> = sequence
                                .as_ref()
                                .to_lowercase()
                                .chars()
                                .filter(|c| !c.is_ascii_whitespace() && !c.is_ascii_punctuation())
                                .collect();
    let length = sequence.len();

    if sequence.len() <= 1 {
        return true;
    }

    if sequence[0] != sequence[length - 1] {
        return false;
    }

    let new_sequence: String = sequence[1..length - 1].into_iter().collect();
    return is_palindrome_recursive(new_sequence);
}

fn is_palindrome(sequence: &str) -> bool {
    let sequence = sequence
                .to_lowercase()
                .chars()
                .filter(|c| !c.is_ascii_whitespace() && !c.is_ascii_punctuation())
                .collect::<String>();
    sequence == sequence.chars().rev().collect::<String>()
}

fn is_palindrome_generic<W: PartialEq + AsRef<str>>(sequence: W) -> bool {
    let sequence = sequence
                    .as_ref()
                    .to_lowercase()
                    .chars()
                    .filter(|c| !c.is_ascii_whitespace() && !c.is_ascii_punctuation())
                    .collect::<String>();
    sequence == sequence.chars().rev().collect::<String>().to_owned()
}

fn main() {
    println!(
        "'racecar' is a palindrome: {:?}",
        is_palindrome_recursive("racecar")
    );
    println!(
        "'racecaR' is a palindrome: {:?}",
        is_palindrome("racecaR")
    );
    println!(
        "'Was it a car or a cat I saw?' is a palindrome: {:?}",
        is_palindrome_generic("Was it a car or a cat I saw?")
    );
}

pub mod test_setup {
    pub fn palindromes() -> Vec<&'static str> {
    vec!["rAcEcaR", "kayak", "a", "Noon", "A man, a plan, a canal, Panama!", "藏ata藏", "ßềaềß",
         "Was it a car or a cat I saw?", "Sit on a potato pan, Otis."]
   }

   pub fn not_palindromes() -> Vec<&'static str> {
    vec!["stoot", "This is not a palindrome.", "racecars"]
   }
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_is_palindrome_recursive() {
        for palindrome in test_setup::palindromes() {
            println!("{:?}", palindrome);
            assert!(is_palindrome_recursive(palindrome));
        }

        for not_palindrome in test_setup::not_palindromes() {
            assert!(!is_palindrome_recursive(not_palindrome));
        }
    }

    #[test]
    fn test_is_palindrome() {
        for palindrome in test_setup::palindromes() {
            assert!(is_palindrome(palindrome));
        }

        for not_palindrome in test_setup::not_palindromes() {
            assert!(!is_palindrome(not_palindrome));
        }
    }

    #[test]
    fn test_is_palindrome_generic() {
        for palindrome in test_setup::palindromes() {
            assert!(is_palindrome_generic(palindrome));
        }

        for not_palindrome in test_setup::not_palindromes() {
            assert!(!is_palindrome_generic(not_palindrome));
        }
    }
}