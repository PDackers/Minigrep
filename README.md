# Minigrep

## Requirements
[Rust-lang](https://www.rust-lang.org/tools/install) should be installed.

## Usage
```sh
./cargo run -- <query> <filename>.txt ignore
```

Example - `./cargo run -- To poem_by_emily_dickinson.txt ignore

### Allowed Options
```
ignore - Case-insensitive
```

## Run tests
```sh
cargo test
```

## Note
This is not a drop in replacement for grep. This is written for the sake of learning Rust.

## Source
This is an excersice from [the experimental book](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html).
