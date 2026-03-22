use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn compile_fail_cases() {
    let cases = [
        "non_enum.rs",
        "named_field_variant.rs",
        "multi_field_variant.rs",
        "duplicate_leaf_in_local.rs",
        "duplicate_leaf_across_union.rs",
    ];

    for case in cases {
        assert_compile_fails(case);
    }
}

fn assert_compile_fails(case_file: &str) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().expect("workspace root");
    let ui_dir = manifest_dir.join("tests/ui");

    let temp = std::env::temp_dir().join(format!(
        "union_error_compile_fail_{}_{}",
        case_file.replace('.', "_"),
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&temp);
    fs::create_dir_all(temp.join("src")).expect("create temp src");

    let dep_path = workspace_root.join("union-error");
    let cargo_toml = format!(
        "[package]\nname = \"compile-fail-case\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nunion-error = {{ path = {:?} }}\n",
        dep_path
    );
    fs::write(temp.join("Cargo.toml"), cargo_toml).expect("write Cargo.toml");

    copy_file(&ui_dir.join(case_file), &temp.join("src/main.rs"));

    if case_file == "duplicate_leaf_across_union.rs" {
        copy_file(
            &ui_dir.join("duplicate_leaf_file1.rs"),
            &temp.join("src/duplicate_leaf_file1.rs"),
        );
        copy_file(
            &ui_dir.join("duplicate_leaf_file2.rs"),
            &temp.join("src/duplicate_leaf_file2.rs"),
        );
    }

    let output = Command::new("cargo")
        .arg("check")
        .arg("--offline")
        .current_dir(&temp)
        .output()
        .expect("run cargo check");

    assert!(
        !output.status.success(),
        "expected `{}` to fail compilation, but it succeeded\nstdout:\n{}\nstderr:\n{}",
        case_file,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(temp);
}

fn copy_file(from: &Path, to: &Path) {
    let content = fs::read(from).expect("read fixture");
    fs::write(to, content).expect("write fixture");
}
