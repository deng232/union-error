Write a README.md for a Rust crate named `error-union`.

The README should be professional, clear, and structured.

Focus on:

1. INTRODUCTION
- Explain that this crate provides a flat, zero-nesting, type-safe error system
- Mention key goals:
  - single centralized error enum
  - no nested error types
  - zero runtime cost (no Box<dyn Error>)
  - automatic conversion using `?`
  - precise error location using track_caller

2. FEATURES
Include sections explaining:
- Flat error model (no AppError::File(FileError::Parse))
- Centralized definition (one enum in error.rs)
- Automatic From<T> generation via derive macro
- Located<T> capturing file/line/column
- No re-wrapping when propagating AppError

3. MECHANISM (VERY IMPORTANT)
Explain clearly how the macro works:

- User writes:
  enum AppError {
      Parse(Located<ParseIntError>),
      Io(Located<std::io::Error>)
  }

- The derive macro generates:
  impl From<T> for AppError with #[track_caller]

- Explain how `?` expands to From::from(...)
- Explain why track_caller captures the correct location
- Explain that Located<T> stores:
  - source error
  - caller location

- Explain error chain:
  AppError -> Located<T> -> T

4. DESIGN PRINCIPLES
- Tree height must be 1
- Only leaf errors are stored
- No nested enums
- No dynamic dispatch
- No extra Display layer

5. STRUCTURE
Explain the workspace layout:
- error-union (runtime crate)
  - Located<T>
- error-union-derive (proc macro)
  - generates From, Display, Error

6. LIMITATIONS
- only tuple variants with 1 field
- must use Located<T>
- no duplicate inner types
- no custom attributes (v1)

7. COMPARISON
Brief comparison with:
- thiserror
- anyhow

8. SUMMARY
Short closing paragraph describing when to use this crate

Style:
- No emojis
- Clean markdown headings
- Concise but complete
- No unnecessary storytelling
