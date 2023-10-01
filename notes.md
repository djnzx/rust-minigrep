#### creating

```shell
cargo new minigrep
```

#### running

```shell
cargo run -- dreary poem.txt
```

```shell
cargo run -- Dreary poem.txt
```

```shell
IGNORE_CASE=1 cargo run -- Dreary poem.txt
```

```shell
cargo run
```

libraries are [at](https://crates.io)

### configuring code formatter

https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=

### `stdout` vs `stderr`

```shell
cargo run > output.txt
```

just `eprintln!` instead of `println!`
