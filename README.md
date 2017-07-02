# Lorem Ipsum

[![](https://img.shields.io/crates/v/lipsum.svg)][crates-io]
[![](https://docs.rs/lipsum/badge.svg)][api-docs]
[![](https://travis-ci.org/mgeisler/lipsum.svg)][travis-ci]
[![](https://ci.appveyor.com/api/projects/status/ku3xlumht6r68f0l?svg=true)][appveyor]

This is small Rust library for generating traditional
mangled-Latin [lorem ipsum filler text][lorem ipsum] for your
application.


## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
lipsum = "0.1"
```

and this to your crate root:
```rust
extern crate lipsum;
```


## Documentation

Please see the **[API documentation][api-docs]**.


## Getting Started

Use the `lipsum` function to generate lorem ipsum text:
```rust
extern crate lipsum;

use lipsum::lipsum;

fn main() {
    // Print 25 random words of lorem ipsum text.
    println!("{}", lipsum(25));
}
```

The text will start with "Lorem ipsum dolor sit amet, …" and will
become random after 18 words. The text is generated using a Markov
chain which is trained on the full text of Cicero's work *De finibus
bonorum et malorum* ("On the ends of good and evil"). The classic
lorem ipsum text is derived from part of that book.


## Release History

### Version 0.1.0 — July 2nd, 2017

First public release.


## License

Lipsum can be distributed according to the [MIT license][mit].
Contributions will be accepted under the same license.


[crates-io]: https://crates.io/crates/lipsum
[api-docs]: https://docs.rs/lipsum/
[lorem ipsum]: https://en.wikipedia.org/wiki/Lorem_ipsum
[travis-ci]: https://travis-ci.org/mgeisler/lipsum
[appveyor]: https://ci.appveyor.com/project/mgeisler/lipsum
[mit]: LICENSE
