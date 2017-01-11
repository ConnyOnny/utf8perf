Benchmark Rust stdlib UTF-8 parser vs [the crate utf8parse](https://crates.io/crates/utf8parse).

# Results

## Latin Lorem Ipsum Text

    Read 403439616 bytes.
    Parser "tab" needed a median 1.225792934 seconds to parse 403439616 characters.
    Parser "std" needed a median 0.246446495 seconds to parse 403439616 characters.

## Russian Lorem Ipsum Text

    Read 376832000 bytes.
    Parser "tab" needed a median 1.287161329 seconds to parse 208273408 characters.
    Parser "std" needed a median 0.514683528 seconds to parse 208273408 characters.
