/// The traditional lorem ipsum text. From
/// https://en.wikipedia.org/wiki/Lorem_ipsum.
const LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
     labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco \
     laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
     voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
     cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

/// Generate a standard lorem ipsum text.
pub fn lipsum() -> String {
    String::from(LOREM_IPSUM)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_lorem_ipsum() {
        assert!(lipsum().starts_with("Lorem ipsum"));
    }
}
