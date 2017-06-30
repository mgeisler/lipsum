extern crate rand;

use std::collections::HashMap;
use rand::Rng;

/// A bigram is simply two consecutive words.
type Bigram<'a> = (&'a str, &'a str);

/// Simple order two Markov chain implementation.
///
/// The [Markov chain] is a chain of order two, which means that it
/// will use the previous two words (a bigram) when predicting the
/// next word. This is normally enough to generate random text that
/// looks somewhat plausible. The implementation is based on
/// [Generating arbitrary text with Markov chains in Rust][blog post].
///
/// [Markov chain]: https://en.wikipedia.org/wiki/Markov_chain
/// [blog post]: https://blakewilliams.me/posts/generating-arbitrary-text-with-markov-chains-in-rust
#[derive(Default)]
pub struct MarkovChain<'a> {
    map: HashMap<Bigram<'a>, Vec<&'a str>>,
}

// Based on
// https://blakewilliams.me/posts/generating-arbitrary-text-with-markov-chains-in-rust
impl<'a> MarkovChain<'a> {
    /// Create a new Markov chain. It will use a default thread-local
    /// random number generator.
    pub fn new() -> MarkovChain<'a> {
        MarkovChain { map: HashMap::new() }
    }

    /// Add new text to the Markov chain. This can be called several
    /// times to build up the chain.
    pub fn learn(&mut self, sentence: &'a str) {
        let words = sentence.split_whitespace().collect::<Vec<&str>>();
        for window in words.windows(3) {
            let (a, b, c) = (window[0], window[1], window[2]);
            self.map.entry((a, b)).or_insert(vec![]).push(c);
        }
    }

    /// Generate `n` words worth of lorem ipsum text. The text will
    /// start from a random point in the Markov chain.
    pub fn generate(&self, n: usize) -> String {
        if self.map.is_empty() {
            // The learn method has not been called.
            return String::new();
        }

        let mut rng = rand::thread_rng();
        let keys = self.map.keys().collect::<Vec<&Bigram>>();
        let (mut a, mut b) = **rng.choose(&keys).unwrap();
        let mut sentence = String::from(a) + " " + b;

        for _ in 0..n {
            while !self.map.contains_key(&(a, b)) {
                let new_key = **rng.choose(&keys).unwrap();
                a = new_key.0;
                b = new_key.1;
            }

            let next_words = &self.map[&(a, b)];
            let c = rng.choose(next_words).unwrap();
            sentence.push(' ');
            sentence.push_str(c);
            a = b;
            b = c;
        }

        return sentence;
    }
}

/// The traditional lorem ipsum text as given in [Wikipedia]. Using
/// this text alone for a Markov chain of order two doesn't work very
/// well since each bigram (two consequtive words) is followed by just
/// one other word. In other words, the Markov chain will always
/// produce the same output and recreate the lorem ipsum text
/// precisely. However, combining it with the full text in
/// [`LIBER_PRIMUS`] works well.
///
/// [Wikipedia]: https://en.wikipedia.org/wiki/Lorem_ipsum
/// [`LIBER_PRIMUS`]: constant.LIBER_PRIMUS.html
const LOREM_IPSUM: &str = include_str!("lorem-ipsum.txt");

/// The first book in Cicero's work De finibus bonorum et malorum ("On
/// the ends of good and evil"). The lorem ipsum text in
/// [`LOREM_IPSUM`] is derived from part of this text.
///
/// [`LOREM_IPSUM`]: constant.LOREM_IPSUM.html
const LIBER_PRIMUS: &str = include_str!("liber-primus.txt");

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

    #[test]
    fn empty_chain() {
        let chain = MarkovChain::new();
        assert_eq!(chain.generate(10), "");
    }

    #[test]
    fn chain_map() {
        let mut chain = MarkovChain::new();
        chain.learn("foo bar baz quuz");
        let map = &chain.map;

        assert_eq!(map.len(), 2);
        assert_eq!(map[&("foo", "bar")], vec!["baz"]);
        assert_eq!(map[&("bar", "baz")], vec!["quuz"]);
    }
}
