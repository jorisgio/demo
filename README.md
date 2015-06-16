# Description

 The code implements a small tile based game. A rover moves on a rectangular
matrix and clean dust (hooray !). The code is a bit over-engineered for small
maps, but is optimized for large map with sparse dust. The map is represented as
a a point [R+tree](https://en.wikipedia.org/wiki/R%2B_tree), a spatial balanced
tree spliting the matrix into non overlapping area. The lookup is thus in O(lg
M) where M is the number of dust tiles.

# Installation and usage

The code is written in rust and should run on Linux and OS X (not tested) with
both rust stable (1.1, not tested) and nightly (1.2). For instructions on how to
install rust, see
[stable](https://doc.rust-lang.org/stable/book/installing-rust.html) or 
[nightly](https://doc.rust-lang.org/nightly/book/installing-rust.html).

It will install [cargo](http://doc.crates.io/), the rust package manager.

To build the code, run 

    cargo build

or, for release (optimized) version

    cargo build --release

This will install a few packages from [crates.io](http://crates.io), but the
only used dependency is the [num crate](https://crates.io/crates/num) which
provides abstraction over numerical types.

To launch the program, either do :

    cargo run

or 

    ./target/debug/demo  

To run the unit tests

    cargo test

To run the integration tests, type

    ./integration-tests.sh


# Implementation notes

 - `geometry.rs` is a small geometric library providing points, rectangles and
   lines representation. It is quite rough would probably benefits from a lot of
improvements. In particular, i used the comparaison traits instead of defining
custom traits (typeclasses) which makes semantic for ordering between objects
unclear and the code less readable.

 - `rtree.rs` is a point R+tree implementation. Requires more tests and more
   unit tests coverage. This proved to be harder than expected.

 - `parser.rs` is the parser for the input format

 - `game.rs` is the game logic.

# Bugs

The code probably still has some bug since i spent only 9hours on it but i spent
3 hours tracking a nasty issue down to the rust compiler, see
[#26339](https://github.com/rust-lang/rust/issues/26339).

