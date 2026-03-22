# error-union

`error-union` is a Rust crate for building a **flat, type-safe error system** with precise call-site tracking.

The crate is designed for applications that want one centralized error enum (for example, `AppError`) without nested error trees, dynamic dispatch (`Box<dyn Error>`), or repeated boilerplate conversion code.

## Introduction

`error-union` focuses on five goals:

1. **Single centralized error enum**: all propagated application errors are listed in one place.
2. **No nested error types**: avoid patterns like `AppError::File(FileError::Parse(...))`.
3. **Zero-cost static typing**: concrete enum variants and concrete source types, not trait-object boxing.
4. **Automatic conversion with `?`**: leaf errors convert into your root enum through generated `From<T>` impls.
5. **Precise source location**: each converted leaf error is wrapped in `Located<T>`, capturing file/line/column using `#[track_caller]`.

## Features

### Flat error model

Your public error type is a single enum where each variant stores exactly one leaf error (via `Located<T>`). This keeps tree height at 1 and error matching straightforward.

### Centralized definition

The enum lives in one module (commonly `error.rs`). Other modules only return `Result<T, AppError>` and use `?`.

### Automatic `From<T>` generation

`#[derive(ErrorUnion)]` generates `From<LeafError>` for each variant, so any `?` on that leaf type automatically maps into your app error.

### `Located<T>` call-site capture

`Located<T>` stores both:

- the original source error `T`
- `&'static std::panic::Location<'static>` for the conversion call site

Its `Display` implementation prints the source message plus `file:line:column`.

### No re-wrapping when propagating your app error

If a function already returns `AppError`, using `?` in a caller returning `AppError` does not re-convert or change location. Only leaf-to-root conversion introduces a new `Located<T>`.

## Mechanism

### 1) You define the root enum

```rust
use error_union::{ErrorUnion, Located};

#[derive(Debug, ErrorUnion)]
pub enum AppError {
    Parse(Located<std::num::ParseIntError>),
    Io(Located<std::io::Error>),
}
```

### 2) The derive macro generates conversion impls

For each variant `Variant(Located<T>)`, the macro generates:

```rust
impl From<T> for AppError {
    #[track_caller]
    fn from(source: T) -> Self {
        Self::Variant(error_union::Located::new(source))
    }
}
```

It also generates `Display` and `std::error::Error` impls for the enum.

### 3) `?` uses `From::from`

When `?` is applied to a `Result<_, T>` in a function returning `Result<_, AppError>`, Rust lowers it to a conversion path using `From<T> for AppError`.

Because generated `from` is `#[track_caller]`, and because it calls `Located::new` (also `#[track_caller]`), the stored location points to the precise `?` call site where conversion happened.

### 4) Error chain shape

The runtime source chain is:

- `AppError`
- `Located<T>`
- `T`

This gives a typed top-level enum and preserved underlying leaf error.

## Design principles

- Tree height is always 1 for app-level variants.
- Variants store leaf errors, not nested intermediate enums.
- No dynamic dispatch is required for propagation.
- No extra display-wrapper layer beyond `Located<T>`.
- Conversion and formatting behavior is generated consistently by derive.

## Workspace structure

This repository is a Cargo workspace with two crates:

- **`error-union`** (runtime crate)
  - exports `Located<T>`
  - re-exports `ErrorUnion` derive macro
- **`error-union-derive`** (proc-macro crate)
  - parses the enum
  - generates `From`, `Display`, and `Error` implementations

## Limitations (v0.1)

Current derive behavior expects a constrained enum shape:

1. **Tuple variants with exactly one field** are required.
2. Intended variant field form is **`Located<T>`**.
3. No extra derive-specific custom attributes are supported.
4. If multiple variants use the same inner leaf type `T`, conflicting `From<T>` impls would result (so inner leaf types should be unique).
