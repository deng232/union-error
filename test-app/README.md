Write a README.md for a Rust example project demonstrating usage of `error-union`.

Focus on usage, not implementation details.

Include:

1. OVERVIEW
- This is a demo app using error-union
- Shows flat error handling and location tracking

2. PROJECT STRUCTURE
Explain files:
- error.rs → defines AppError and Result<T>
- file1.rs → produces parse errors
- file2.rs → produces IO errors and calls file1
- main.rs → runs everything and prints errors

3. DEFINING THE ERROR

Show example:

#[derive(Debug, ErrorUnion)]
pub enum AppError {
    Parse(Located<std::num::ParseIntError>),
    Io(Located<std::io::Error>),
}

Explain:
- All errors are defined here
- No other modules define error types

4. USING RESULT

Explain:
- All functions return crate::error::Result<T>
- `?` automatically converts errors into AppError

5. EXAMPLES

Explain behavior:

Case 1: parse failure
- calling parse_number("abc")
- error is AppError::Parse
- location points to file1.rs at parse()?

Case 2: IO failure
- missing file
- error is AppError::Io
- location points to read_to_string()?

Case 3: nested call
- file2 calls file1
- file1 returns AppError
- file2 does NOT wrap again
- location stays in file1.rs

6. IMPORTANT RULES

- Always return Result<T>
- Never define new error enums in modules
- Never wrap AppError again
- Do not use unwrap()

7. RUNNING

cargo run

8. SUMMARY

Explain:
- flat error propagation
- precise location tracking
- centralized error design

Style:
- Clear, instructional
- Minimal theory
- Focus on how to use
- No emojis
