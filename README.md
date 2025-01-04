# Minigrep

This is my fist Rust programm that imitates grep inspired by [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

## Building

```bash
cargo build --release
```

## Running

```bash
./minigrep -w <some word> -f <path_to_file> [-ic]
```

**Arguments:**

- --word / -w - pattern to search
- --file / -f - file to scan
- (optional) --ignore_case / -ic - flag to ignore case (default: false)


**For example**

```bash
./target/release/minigrep -w rust -f data/test.txt -ic
```
