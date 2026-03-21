# error-union crate scaffold for Codex

This bundle gives you a **small, testable starting point** for a crate that provides:

- `Located<E>`: wraps a leaf error with `#[track_caller]` location
- `#[derive(ErrorUnion)]`: generates `From<T>`, `Display`, and `Error` for a flat enum
- one centralized flat error enum in the app crate
- no nested runtime error wrappers beyond `Located<E>`

## Why the workspace has 2 crates

Procedural macros must live in a separate crate with `[lib] proc-macro = true`, and cannot be used from the same crate that defines them. Rust derive macros can also declare helper attributes if needed later. See the Rust Reference. citeturn415405search0turn415405search2

## What to ask Codex to do next

Feed Codex this folder and ask it to do these steps in order:

1. **Make the scaffold compile.**
   - Keep the API shape.
   - Do not add flattening yet.
   - Keep only tuple enum variants with exactly one field.

2. **Implement the derive macro `ErrorUnion`.**
   - Input: an enum like:
     ```rust
     #[derive(Debug, ErrorUnion)]
     pub enum AppError {
         Parse(std::num::ParseIntError),
         Io(std::io::Error),
     }
     ```
   - Generate:
     - `impl From<T> for AppError` with `#[track_caller]`
     - `impl From<error_union::Located<T>> for AppError`
     - `impl Display for AppError` by delegating to the inner `Located<T>`
     - `impl std::error::Error for AppError`
   - Reject:
     - structs
     - non-enums
     - unit variants
     - named-field variants
     - multi-field variants
     - duplicate inner source types

3. **Decide whether the derive rewrites storage or not.**
   Pick one of these and keep it consistent:
   - **Option A, easier:** user writes `Parse(Located<ParseIntError>)`
   - **Option B, nicer API:** user writes `Parse(ParseIntError)` and the derive generates conversion impls only; the enum storage stays as written.

   I recommend **Option A first** because derive macros cannot replace the original enum item with a modified enum definition; derive output is appended after the item. If you want rewritten storage, switch to an attribute macro later. Rust derive macros append new items after the annotated item rather than replacing it. citeturn415405search0turn415405search2

4. **Add tests.**
   Add these tests in `error-union/tests/` or inside the runtime crate:
   - `?` captures the exact call site line
   - `Display` includes the inner error message and the file/line/column
   - `source()` returns the wrapped original error
   - duplicate source types fail to compile
   - invalid variant shapes fail to compile

5. **Use `trybuild` for compile-fail tests.**
   Add a `tests/ui` directory with bad inputs.

## Suggested v1 constraints

- Only support enums.
- Only support tuple variants with one field.
- Do not support automatic recursive flattening yet.
- Keep the API flat and centralized.
- Every leaf error must be wrapped in `Located<E>`.

## Example app-side usage target

```rust
use error_union::ErrorUnion;

#[derive(Debug, ErrorUnion)]
pub enum AppError {
    Parse(error_union::Located<std::num::ParseIntError>),
    Io(error_union::Located<std::io::Error>),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

Then this should work:

```rust
fn parse_num(s: &str) -> Result<u32> {
    let n = s.parse::<u32>()?;
    Ok(n)
}
```

## Later extensions, after v1 works

- helper attributes like `#[transparent]`
- custom `Display` messages
- an attribute macro that can rewrite `Parse(ParseIntError)` into `Parse(Located<ParseIntError>)`
- flattening/unioning support for helper enums
