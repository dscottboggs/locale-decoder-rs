#[derive(Debug, PartialEq)]
pub struct Locale {
    pub language: String,
    pub country: Option<String>,
    pub encoding: Option<String>,
    pub modifier: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("found unexpected character '{0}' in \"{2}\" at {1}")]
    UnexpectedCharacterRecieved(char, usize, String),
}
pub type Result<T> = std::result::Result<T, Error>;

impl Locale {
    pub fn parse<'a, T: AsRef<str>>(text: T) -> Result<Locale> {
        // lang_COUNTRY.ENCODING@MODIFIER
        let mut locale = Locale {
            language: String::new(),
            country: None,
            encoding: None,
            modifier: None,
        };
        let text = text.as_ref();
        for (idx, character) in text.chars().enumerate() {
            match character {
                '_' => {
                    if locale.country.is_some()
                        || locale.encoding.is_some()
                        || locale.modifier.is_some()
                    {
                        return Err(Error::UnexpectedCharacterRecieved(
                            character,
                            idx,
                            text.to_string(),
                        ));
                    }
                    locale.country = Some(String::new());
                }
                '.' => {
                    if locale.encoding.is_some() || locale.modifier.is_some() {
                        return Err(Error::UnexpectedCharacterRecieved(
                            character,
                            idx,
                            text.to_string(),
                        ));
                    }
                    locale.encoding = Some(String::new());
                }
                '@' => {
                    if locale.modifier.is_some() {
                        return Err(Error::UnexpectedCharacterRecieved(
                            character,
                            idx,
                            text.to_string(),
                        ));
                    }
                    locale.modifier = Some(String::new());
                }
                character => match (&locale.country, &locale.encoding, &locale.modifier) {
                    (None, None, None) => locale.language.push(character),
                    (Some(country), None, None) => {
                        locale.country = Some(format!("{country}{character}"))
                    }
                    (_, Some(encoding), None) => {
                        locale.encoding = Some(format!("{encoding}{character}"))
                    }
                    (_, _, Some(modifier)) => {
                        locale.modifier = Some(format!("{modifier}{character}"))
                    }
                },
            }
        }
        Ok(locale)
    }
}
impl TryFrom<String> for Locale {
    fn try_from(text: String) -> Result<Self> {
        Locale::parse(text)
    }
    type Error = Error;
}

#[cfg(test)]
mod tests {
    use super::{Locale, Result};
    #[test]
    fn parse_full() -> Result<()> {
        let result = super::Locale::parse("lang_COUNTRY.ENCODING@MODIFIER")?;
        assert_eq!(result.language, "lang");
        assert_eq!(result.country, Some("COUNTRY".into()));
        assert_eq!(result.encoding, Some("ENCODING".into()));
        assert_eq!(result.modifier, Some("MODIFIER".into()));
        Ok(())
    }

    #[test]
    fn parse_rest() {
        let tests = vec![
            (
                "lang_COUNTRY",
                Locale {
                    language: "lang".into(),
                    country: Some("COUNTRY".into()),
                    encoding: None,
                    modifier: None,
                },
            ),
            (
                "lang@MODIFIER",
                Locale {
                    language: "lang".into(),
                    country: None,
                    encoding: None,
                    modifier: Some("MODIFIER".into()),
                },
            ),
            (
                "justLang",
                Locale {
                    language: "justLang".into(),
                    country: None,
                    encoding: None,
                    modifier: None,
                },
            ),
        ];
        for (example, expected_result) in tests {
            assert_eq!(Locale::parse(example).unwrap(), expected_result);
        }
    }
}
