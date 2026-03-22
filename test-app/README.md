# demo-app: `union-error` usage example

This example application demonstrates how to use `union-error` in a real module layout with one centralized `AppError`, flat propagation, and call-site location tracking.

## Overview

The app intentionally triggers parse and I/O paths to show how `?` automatically converts leaf errors into `AppError` variants while preserving where conversion happened.

## Project structure

- `src/error.rs`  
  Defines `AppError` and crate-wide `Result<T>` alias.

- `src/file1.rs`  
  Contains parse logic and produces parse failures.

- `src/file2.rs`  
  Performs file I/O and then calls `file1`, demonstrating both direct and delegated propagation.

- `src/main.rs`  
  Runs the flow, prints top-level error, and walks the `source()` chain.

## Defining the error

In `src/error.rs`:

```rust
use union_error::{ErrorUnion, Located};

#[derive(Debug, ErrorUnion)]
pub enum AppError {
    Parse(Located<std::num::ParseIntError>),
    Io(Located<std::io::Error>),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

Key rules:

- All propagated app errors are listed in this single enum.
- Other modules do not define additional propagation enums.
- Variants use `Located<T>` so location is captured when conversion occurs.

## Using `Result`

All fallible functions in the app return `crate::error::Result<T>`. This guarantees `?` can convert leaf errors directly into `AppError`.

Example in `file1.rs`:

```rust
pub fn parse_number(input: &str) -> Result<u32> {
    let value = input.parse::<u32>()?;
    Ok(value)
}
```

Example in `file2.rs`:

```rust
pub fn read_and_parse(path: &str) -> Result<u32> {
    let text = std::fs::read_to_string(path)?;
    let value = file1::parse_number(text.trim())?;
    Ok(value)
}
```

## Behavior examples

### Case 1: Parse failure

If `parse_number("abc")` is called, `input.parse::<u32>()?` fails with `ParseIntError`, which becomes `AppError::Parse`. The location in `Located<_>` points to the `?` site in `file1.rs`.

### Case 2: I/O failure

If `read_and_parse("missing.txt")` is called, `read_to_string(path)?` fails with `std::io::Error`, which becomes `AppError::Io`. The location points to the `?` site in `file2.rs`.

### Case 3: Nested call without re-wrapping

`file2::read_and_parse` calls `file1::parse_number(...)?`. When `file1` already returns `AppError`, `file2` propagates that same `AppError` without adding a second wrapper, so the recorded parse location remains in `file1.rs`.

## Important rules

1. Always return `crate::error::Result<T>` from fallible app functions.
2. Keep the propagated enum centralized in `src/error.rs`.
3. Do not create intermediate nested propagation enums.
4. Avoid `unwrap()` in fallible paths; use `?` to preserve typed propagation.

## Running

From the workspace root:

```bash
cargo run -p demo-app
```

You can create a `number.txt` file in `test-app/` to exercise the successful path, or run without it to observe `Io` handling.

## Summary

This demo shows the intended usage pattern for `union-error`: one flat `AppError`, automatic `?` conversions from leaf errors, and precise conversion-site diagnostics through `Located<T>`.
