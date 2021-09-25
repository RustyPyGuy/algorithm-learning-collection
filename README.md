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
chapters in the code and pseudocode is implemented as fair use of the text.

# Included Algorithms
- Merge Sort of a Vector using the borrowing model in Rust
- Merge Sort of a Vector by passing ownership in Rust
- Merge Sort of a Vector of coordinate pairs (vector of 2-element arrays) on either x or y value
- Closest Pair Algorithm using a divide and conquer approach for selecting the
two pairs in a coordinate vector with the smallest distance
*NOTE: Closest Pair Algorithm is currently failing tests. Still working on this one.*
- Test functions to check that the above algorithms are working as designed using randomized data and more primitive processor-expensive algorithms for comparison.

# Use
Prior to first build:
`cargo update`

To run the multiple algorithms with timing details output to console.

`cargo run --release` The release flag is important unless you want to run really slow. By default it will print out timings of execution so that you can see the timed progression of execution time of increased data.

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
*I believe in strong open source licenses, even for small projects like this.*

The Algorithm learning collection, a set of basic algorithms for improving programming skills and understanding of computer science principles, Copyright (C) 2021 github user: RustyPyGuy

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

See the file LICENSE for full details of the GPL v3.0.
