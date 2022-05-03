use regex::Regex;
use unidecode::unidecode;

lazy_static! {
    /// The regex used to slugify our inputs
    static ref REPLACEABLE: Regex = Regex::new(r"\s+|\-+").unwrap();
}

/// This method transforms a string into a slug to request the api
///
/// # Args
///
/// * raw_str : The string to transform as slug
///
/// # Examples
///
/// ```
/// assert_eq!(slugify(" La Défense ", "la-défense"));
/// ```
pub(crate) fn slugify(raw_str: &str) -> String {
    return unidecode(
        REPLACEABLE
            .replace_all(raw_str.to_lowercase().trim(), "+")
            .to_string()
            .as_str(),
    );
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_slugify_trim() {
        assert_eq!("foo", slugify(" foo    "));
        assert_eq!("fo+o", slugify(" fo     o    "));
    }

    #[test]
    fn test_slugify_lowercase() {
        assert_eq!("foo", slugify(" fOo    "));
        assert_eq!("foo", slugify("fOO"));
        assert_eq!("foo", slugify("FOO"));
        assert_eq!("foo", slugify("FoO"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!("la+defense", slugify("La Défense"));
        assert_eq!("paris+saint+lazare", slugify("Paris-Saint-Lazare"));
        assert_eq!(
            "la+defense+(grande+arche)",
            slugify("La Défense (Grande Arche)")
        );
        assert_eq!("issy+++val+de+seine", slugify("Issy - Val de Seine"));
        assert_eq!(
            "henri+farman+(porte+de+seine)",
            slugify("Henri Farman (Porte de Seine)")
        );
        assert_eq!(
            "henri+farman+(porte+de+seine)",
            slugify("    Henri Farman (Porte de Seine)         "),
            "with leading trailing"
        );
    }
}
