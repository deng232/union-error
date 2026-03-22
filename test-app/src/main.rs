mod error;
mod file1;
mod file2;

use crate::error::Result;

fn run() -> Result<()> {
    let n1 = file1::parse_number("123")?;
    println!("parsed from file1: {n1}");

    let n2 = file2::read_and_parse("number.txt")?;
    println!("parsed from file2: {n2}");

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");

        let mut source = std::error::Error::source(&err);
        while let Some(err) = source {
            eprintln!("caused by: {err}");
            source = err.source();
        }

        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::error::AppError;
    use super::{file1, file2};

    fn write_temp(contents: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        path.push(format!("union_error_test_{nonce}.txt"));
        std::fs::write(&path, contents).expect("temp file write should succeed");
        path
    }

    #[test]
    fn parse_error_points_to_file1_parse_site() {
        let err = file1::parse_number("abc").expect_err("parsing should fail");
        match err {
            AppError::Parse(located) => {
                assert!(located.location().file().ends_with("src/file1.rs"));
                assert_eq!(located.location().line(), 11);
            }
            other => panic!("expected parse variant, got {other:?}"),
        }
    }

    #[test]
    fn io_error_points_to_file2_read_site() {
        let path = std::env::temp_dir().join("definitely_missing_union_error_file.txt");
        let _ = std::fs::remove_file(&path);
        let err =
            file2::read_and_parse(path.to_str().expect("utf8 path")).expect_err("read should fail");
        match err {
            AppError::Io(located) => {
                assert!(located.location().file().ends_with("src/file2.rs"));
                assert_eq!(located.location().line(), 11);
            }
            other => panic!("expected io variant, got {other:?}"),
        }
    }

    #[test]
    fn file2_parse_failure_keeps_file1_location_without_rewrap() {
        let path = write_temp("xyz");
        let err = file2::read_and_parse(path.to_str().expect("utf8 path"))
            .expect_err("parse should fail");
        let _ = std::fs::remove_file(path);

        match err {
            AppError::Parse(located) => {
                assert!(located.location().file().ends_with("src/file1.rs"));
                assert_eq!(located.location().line(), 11);
            }
            other => panic!("expected parse variant, got {other:?}"),
        }
    }

    #[test]
    fn app_error_is_flat_without_module_wrapper_variants() {
        let err = file1::parse_number("abc").expect_err("parsing should fail");
        match err {
            AppError::Parse(_) => {}
            _ => panic!("expected Parse leaf variant"),
        }

        let debug = format!("{err:?}");
        assert!(debug.starts_with("Parse("));
        assert!(!debug.contains("File1("));
        assert!(!debug.contains("File2("));
    }
}
