use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<Self, String> {
        let is_empty = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;

        let forbiden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbiden_characters = name.chars().any(|c| forbiden_characters.contains(&c));

        if is_empty || is_too_long || contains_forbiden_characters {
            Err(format!("{} is not a valid subscriber name", name))
        } else {
            Ok(Self(name))
        }
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_ok, assert_err};

    use super::*;

    #[test]
    fn a_256_grapheme_long_name_is_valid_test() {
        let name = "Ñ".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected_test() {
        let name = "Ñ".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected_test() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_test() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected_test() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully_test() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
