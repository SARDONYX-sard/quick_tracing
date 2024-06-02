# Quick Tracing

tracing initialization library to easily get started with tracing.

```rust
fn main() -> std:io::Result<()> {
     let _guard = quick_tracing::init()?;
     tracing::info!("Hey!");
     Ok(())
 }
```

## Derive

Need to write the following it in Cargo.toml

```toml
features = ["derive"]
```

### Attributes

- `test`: Sets the test name for the logger. Log output will be written to a file named `../../logs/{test_name}.log`.
- `file`: Sets the file path for the log output.
- `stdio`: Enables standard I/O output for the logger.(default: true)
- `level`: Sets the log level filter (e.g., `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`).

### Examples

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
