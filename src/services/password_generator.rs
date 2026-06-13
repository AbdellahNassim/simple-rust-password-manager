use rand::Rng;
use rand::thread_rng;
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const DIGITS: &str = "0123456789";

const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}";

pub fn validate_length(length: usize) -> Result<(), String> {
    if length < 8 {
        return Err("Minimum length is 8".to_string());
    }

    Ok(())
}

pub fn generate_password(length: usize, no_symbols: bool) -> String {
    let mut charset = String::new();
    charset.push_str(LOWERCASE);
    charset.push_str(UPPERCASE);
    charset.push_str(DIGITS);
    if !no_symbols {
        charset.push_str(SYMBOLS);
    }
    let chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..chars.len());

            chars[index]
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_password_has_correct_length() {
        let password = generate_password(24, true);

        assert_eq!(password.len(), 24);
    }

    #[test]
    fn generated_password_has_correct_characters() {
        let password = generate_password(24, true);

        assert!(password.chars().all(|c| LOWERCASE.contains(c)
            || UPPERCASE.contains(c)
            || DIGITS.contains(c)
            || SYMBOLS.contains(c)));
    }

    #[test]
    fn reject_short_passwords() {
        assert!(validate_length(4).is_err());
    }
}
