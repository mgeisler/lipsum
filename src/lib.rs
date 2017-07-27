//! Lorem ipsum generator.
//!
//! This crate contains functions for generating pseudo-Latin lorem
//! ipsum placeholder text. The traditional lorem ipsum text start
//! like this:
//!
//! > Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
//! > eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
//! > enim ad minim veniam, quis nostrud exercitation ullamco laboris
//! > nisi ut aliquip ex ea commodo consequat. [...]
//!
//! This text is in the [`LOREM_IPSUM`] constant. Random text looking
//! like the above can be generated using the [`lipsum`] function.
//! This function allows you to generate as much text as desired and
//! each invocation will generate different text. This is done using a
//! [Markov chain] based on both the [`LOREM_IPSUM`] and
//! [`LIBER_PRIMUS`] texts. The latter constant holds the full text of
//! the first book of a work by Cicero, of which the lorem ipsum text
//! is a scrambled subset.
//!
//! The random looking text is generatd using a Markov chain of order
//! two, which simply means that the next word is based on the
//! previous two words in the input texts. The Markov chain can be
//! used with other input texts by creating an instance of
//! [`MarkovChain`] and calling its [`learn`] method.
//!
//! [`LOREM_IPSUM`]: constant.LOREM_IPSUM.html
//! [`LIBER_PRIMUS`]: constant.LIBER_PRIMUS.html
//! [`lipsum`]: fn.lipsum.html
//! [`MarkovChain`]: struct.MarkovChain.html
//! [`learn`]: struct.MarkovChain.html#method.learn
//! [Markov chain]: https://en.wikipedia.org/wiki/Markov_chain

extern crate rand;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use rand::Rng;

/// A bigram is simply two consecutive words.
pub type Bigram<'a> = (&'a str, &'a str);

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

impl<'a> MarkovChain<'a> {
    /// Create a new Markov chain. It will use a default thread-local
    /// random number generator.
    pub fn new() -> MarkovChain<'a> {
        MarkovChain { map: HashMap::new() }
    }

    /// Add new text to the Markov chain. This can be called several
    /// times to build up the chain.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipsum::MarkovChain;
    ///
    /// let mut chain = MarkovChain::new();
    /// chain.learn("red green blue");
    /// assert_eq!(chain.words(("red", "green")), Some(&vec!["blue"]));
    ///
    /// chain.learn("red green yellow");
    /// assert_eq!(chain.words(("red", "green")), Some(&vec!["blue", "yellow"]));
    /// ```
    pub fn learn(&mut self, sentence: &'a str) {
        let words = sentence.split_whitespace().collect::<Vec<&str>>();
        for window in words.windows(3) {
            let (a, b, c) = (window[0], window[1], window[2]);
            self.map.entry((a, b)).or_insert_with(Vec::new).push(c);
        }
    }

    /// Get the possible words following the given bigram, or `None`
    /// if the state is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipsum::MarkovChain;
    ///
    /// let mut chain = MarkovChain::new();
    /// chain.learn("red green blue");
    /// assert_eq!(chain.words(("red", "green")), Some(&vec!["blue"]));
    /// assert_eq!(chain.words(("foo", "bar")), None);
    /// ```
    pub fn words(&self, state: Bigram<'a>) -> Option<&Vec<&str>> {
        self.map.get(&state)
    }

    /// Generate `n` words worth of lorem ipsum text. The text will
    /// start from a random point in the Markov chain.
    ///
    /// See [`generate_from`] if you want to control the starting
    /// point for the generated text.
    ///
    /// # Examples
    ///
    /// Generating the sounds of a grandfather clock:
    ///
    /// ```
    /// use lipsum::MarkovChain;
    ///
    /// let mut chain = MarkovChain::new();
    /// chain.learn("Tick, Tock, Tick, Tock, Ding! Tick, Tock, Ding! Ding!");
    /// println!("{}", chain.generate(15));
    /// ```
    ///
    /// The output looks like this:
    ///
    /// > Ding! Tick, Tock, Tick, Tock, Ding! Ding! Tock, Ding! Tick,
    /// > Tock, Tick, Tock, Tick, Tock
    ///
    /// [`generate_from`]: struct.MarkovChain.html#method.generate_from
    pub fn generate(&self, n: usize) -> String {
        join_words(self.iter().take(n))
    }

    /// Generate `n` words worth of lorem ipsum text. The text will
    /// start from the given bigram.
    ///
    /// Use [`generate`] if the starting point is not important.
    ///
    /// [`generate`]: struct.MarkovChain.html#method.generate
    pub fn generate_from(&self, n: usize, from: Bigram<'a>) -> String {
        join_words(self.iter_from(from).take(n))
    }

    /// Make a never-ending iterator over the words in the Markov
    /// chain. The iterator starts at a random point in the chain.
    pub fn iter(&self) -> Words {
        let keys = self.map.keys().collect::<Vec<&(&str, &str)>>();
        let state = if keys.is_empty() {
            ("", "")
        } else {
            let mut rng = rand::thread_rng();
            **rng.choose(&keys).unwrap()
        };
        Words { map: &self.map, keys: keys, state: state }
    }

    /// Make a never-ending iterator over the words in the Markov
    /// chain. The iterator starts at the given bigram.
    pub fn iter_from(&self, from: Bigram<'a>) -> Words {
        let keys = self.map.keys().collect::<Vec<&(&str, &str)>>();
        Words { map: &self.map, keys: keys, state: from }
    }
}

pub struct Words<'a> {
    map: &'a HashMap<Bigram<'a>, Vec<&'a str>>,
    keys: Vec<&'a Bigram<'a>>,
    state: Bigram<'a>,
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.map.is_empty() {
            return None;
        }

        let result = Some(self.state.0);
        let mut rng = rand::thread_rng();

        while !self.map.contains_key(&self.state) {
            self.state = **rng.choose(&self.keys).unwrap();
        }
        let next_words = &self.map[&self.state];
        let next = rng.choose(next_words).unwrap();
        self.state = (self.state.1, next);
        result
    }
}

fn join_words<'a, I: Iterator<Item = &'a str>>(mut words: I) -> String {
    match words.next() {
        None => String::new(),
        Some(word) => {
            let mut sentence = String::from(word);
            for word in words {
                sentence.push(' ');
                sentence.push_str(word);
            }
            sentence
        }
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
pub const LOREM_IPSUM: &'static str = include_str!("lorem-ipsum.txt");

/// The first book in Cicero's work De finibus bonorum et malorum ("On
/// the ends of good and evil"). The lorem ipsum text in
/// [`LOREM_IPSUM`] is derived from part of this text.
///
/// [`LOREM_IPSUM`]: constant.LOREM_IPSUM.html
pub const LIBER_PRIMUS: &'static str = include_str!("liber-primus.txt");

lazy_static! {
    /// Markov chain generating lorem ipsum text.
    static ref LOREM_IPSUM_CHAIN: MarkovChain<'static> = {
        let mut chain = MarkovChain::new();
        chain.learn(LOREM_IPSUM);
        chain.learn(LIBER_PRIMUS);
        chain
    };
}

/// Generate `n` words of lorem ipsum text. The output starts with
/// "Lorem ipsum" and continues with the standard lorem ipsum text
/// from [`LOREM_IPSUM`]. The text will become random if sufficiently
/// long output is requested.
///
/// [`LOREM_IPSUM`]: constant.LOREM_IPSUM.html
pub fn lipsum(n: usize) -> String {
    LOREM_IPSUM_CHAIN.generate_from(n, ("Lorem", "ipsum"))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_lorem_ipsum() {
        assert_eq!(&lipsum(10)[..11], "Lorem ipsum");
    }

    #[test]
    fn generate_zero_words() {
        assert_eq!(lipsum(0).split_whitespace().count(), 0);
    }

    #[test]
    fn generate_one_word() {
        assert_eq!(lipsum(1).split_whitespace().count(), 1);
    }

    #[test]
    fn generate_two_words() {
        assert_eq!(lipsum(2).split_whitespace().count(), 2);
    }

    #[test]
    fn empty_chain() {
        let chain = MarkovChain::new();
        assert_eq!(chain.generate(10), "");
    }

    #[test]
    fn generate_from() {
        let mut chain = MarkovChain::new();
        chain.learn("red orange yellow green blue indigo violet");
        assert_eq!(chain.generate_from(5, ("orange", "yellow")),
                   "orange yellow green blue indigo");
    }

    #[test]
    fn generate_last_bigram() {
        // The bigram "yyy zzz" will not be present in the Markov
        // chain's map, and so we will not generate "xxx yyy zzz" as
        // one would expect. The chain moves from state "xxx yyy" to
        // "yyy zzz", but sees that as invalid state and resets itsel
        // back to "xxx yyy".
        let mut chain = MarkovChain::new();
        chain.learn("xxx yyy zzz");
        // We use assert! instead of assert_ne! to support early
        // versions of Rust.
        assert!(chain.generate_from(3, ("xxx", "yyy")) != "xxx yyy zzz");
    }

    #[test]
    fn generate_from_no_panic() {
        // No panic when asked to generate a chain from a starting
        // point that doesn't exist in the chain.
        let mut chain = MarkovChain::new();
        chain.learn("foo bar baz");
        chain.generate_from(3, ("xxx", "yyy"));
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
