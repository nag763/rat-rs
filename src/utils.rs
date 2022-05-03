use regex::Regex;

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
    return REPLACEABLE
        .replace_all(raw_str.to_lowercase().trim(), "+")
        .to_string();
}

