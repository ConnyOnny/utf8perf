Benchmark Rust stdlib UTF-8 parser vs (the crate utf8parse)[https://crates.io/crates/utf8parse].

# Results

## Latin Lorem Ipsum Text

    Read 403439616 bytes.
    Parser "table" needed a median 1.722982912 seconds to parse 403439616 characters.
    Parser "stdlib" needed a median 1.239925029 seconds to parse 403439616 characters.

## Russian Lorem Ipsum Text

    Read 376832000 bytes.
    Parser "table" needed a median 1.871329477 seconds to parse 208273408 characters.
    Parser "stdlib" needed a median 0.946937234 seconds to parse 208273408 characters.
