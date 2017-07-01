/// The traditional lorem ipsum text. From
/// https://en.wikipedia.org/wiki/Lorem_ipsum.
const LOREM_IPSUM: &str = include_str!("lorem-ipsum.txt");

/// Generate a standard lorem ipsum text.
pub fn lipsum() -> String {
    String::from(LOREM_IPSUM)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_lorem_ipsum() {
        assert_eq!(&lipsum()[..11], "Lorem ipsum");
    }
}
