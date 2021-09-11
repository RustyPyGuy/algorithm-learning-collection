# Introduction
The algorithm learning collection is a project with the dual purpose of
learning basic computer science algorithms and learning to program in the Rust
language. Consider it akin to doing a bunch of programming lab homework by an
advanced beginner programmer. It includes experiments in timing algorithms and
an attempt at good coding practices, such as unit tests, and managing the code
here on GitHub.  It is not lost on me that I am reinventing many wheels here.
That's somewhat the point.  I attempt use features specific to Rust such as
intentionally distinguishing of ownership of variables and borrowing of
variables to test ideas.  Please be kind with critiques. :-)

# Source Material, Inspiration
Selected algorithms are based on lessons and pseudocode from: 

####  *Algorithms Illuminated Part 1: The Basics* by Tim Roughgarden 

This is an excellent book. Where appropriate I give references to the book
chapters in the code and pseudocode is implemented as fair use.

# Included Algorithms
- Merge Sort of a Vector using the borrowing model in Rust
- Merge Sort of a Vector by passing ownership in Rust
- Merge Sort of a Vector of coordinate pairs (vector of 2-element arrays) on either x or y value
- Closest Pair Algorithm using a divide and conquer approach for selecting the
two pairs in a coordinate vector with the smallest distance
*NOTE: Closest Pair Algorithm is currently failing tests*
- Test functions to check that the above algorithms are working as designed using randomized data and more primitive processor-expensive algorithms for comparison.

# Use
Prior to first build:
`cargo update`

To run the multiple algorithms with timing details output to console.

`cargo run --release` The release flag is important unless you want to run really slow. By default it will print out timings of execution.

To run the test suite:
`cargo test`

# TODO:
This is an ongoing project as I continue my study.  Some general areas of improvement:
- More Algorithms!
- Repair broken algorithms not passing tests.
- Improve error handling.
- Miscellaneous Performance improvements of subroutines.
- Emulate a library for distribution as a crate.
- Detailed TODO items are included in source files.


# License
I believe that software licenses protect the author from legal liability and
create a community understanding of the purpose of its use and have the
potential to increase visibility, development, and collaboration.  This code is
released under the LGPL license.  It is meant for learning and practice, and I
want it to be free.  If somehow you think this code is useful under a more
permissive license, I would be a a bit confused and surprised that anyone would
find much use for these basic things, but I would welcome your contact.  -RustyPyGuy
