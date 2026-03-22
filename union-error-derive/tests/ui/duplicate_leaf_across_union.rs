#[path = "duplicate_leaf_file1.rs"]
mod file1;
#[path = "duplicate_leaf_file2.rs"]
mod file2;

use union_error::error_union;

#[error_union]
enum AppError {
    File1(crate::file1::LocalErrors),
    File2(crate::file2::LocalErrors),
}

fn main() {}
