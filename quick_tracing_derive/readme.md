# Quick Tracing derive

This is derive. See [quick_tracing](https://crates.io/crates/quick_tracing)

Need to write the following it in Cargo.toml

```toml
quick_tracing_derive = { version = "0.1.1" }
```

## Attributes

- `test`: Sets the test name for the logger. Log output will be written to a file named `../../logs/{test_name}.log`.
- `file`: Sets the file path for the log output.
- `stdio`: Enables standard I/O output for the logger.(default: true)
- `level`: Sets the log level filter (e.g., `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`).

## Examples

```rust
#[quick_tracing::init]
fn main() {
    tracing::debug!("Hello, world!");
}
```

- Debug mode + Output file

  If there is no parent directory, it will automatically create one.

> [!WARNING]
> Panic is a possibility.

```rust
#[quick_tracing::init(level= "DEBUG", file = "./log/test.log", stdio = false)]
fn main() {
    tracing::debug!("Hello, world!");
}
```
