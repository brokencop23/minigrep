# Minigrep

This is my fist Rust programm that imitates grep

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
