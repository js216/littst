# littst: Literate Programming with Typst

This tool distills Norman Ramsey's [noweb](https://www.cs.tufts.edu/~nr/noweb/)
into just two short Rust programs: `tangle` extracts the source code, while
`weave` produces a [Typst](https://github.com/typst/typst)-formatted document.

### What is Literate Programming?

Literate programming, introduced in an 1984
[article](http://www.literateprogramming.com/knuthweb.pdf) by Donald Knuth, is a
way of writing programs as explanatory documents for humans first and computers
second. Instead of arranging code solely for the compiler, you interleave prose
and named code chunks in a logical, narrative order; a tool then "tangles" the
chunks into real source files and "weaves" the prose and code into a nicely
formatted document.

### Getting Started

If you have a [Rust](https://rustup.rs/) toolchain, install like this:

    git clone https://github.com/js216/littst.git
    cd littst
    cargo install --path .            # this package (littst)
    cargo install --locked typst-cli  # typst if you don't have it

We can now use `tangle` to process the example input file
[test.nw](examples/test.nw) and extract from it two files,
[main.c](examples/main.c) and [gpio.v](examples/gpio.v):

    cd examples
    tangle main.c < test.nw
    tangle gpio.v < test.nw

To get the literate PDF document, run `weave`:

    weave < test.nw > test.typ
    typst compile test.typ

When the Typst-formatted document ([`test.pdf`](examples/test.pdf)) is produced,
the code chunks will receive syntax formatting appropriate for each file type
(e.g. `.c` file is C and `.v` is Verilog):

### Author

Jakob Kastelic (Stanford Research Systems, Inc.)
