use std::path::Path;

fn check_utf8_no_bom(path: &Path) {
    let content = std::fs::read(path)
        .unwrap_or_else(|e| panic!("Не удалось прочитать {}: {e}", path.display()));
    assert!(
        !content.starts_with(&[0xEF, 0xBB, 0xBF]),
        "{} содержит BOM в начале",
        path.display()
    );
    assert!(
        std::str::from_utf8(&content).is_ok(),
        "{} не валидный UTF-8",
        path.display()
    );
}

#[test]
fn test_all_rust_files_utf8_no_bom() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for entry in walkdir::WalkDir::new(&src_dir) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |e| e == "rs") {
            check_utf8_no_bom(entry.path());
        }
    }
}

#[test]
fn test_all_json_files_utf8_no_bom() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for dir in &["locales", "demo"] {
        let path = root.join(dir);
        if path.exists() {
            for entry in walkdir::WalkDir::new(&path) {
                let entry = entry.unwrap();
                if entry.path().extension().map_or(false, |e| e == "json") {
                    check_utf8_no_bom(entry.path());
                }
            }
        }
    }
}

#[test]
fn test_opencode_json_utf8_no_bom() {
    check_utf8_no_bom(&Path::new(env!("CARGO_MANIFEST_DIR")).join("opencode.json"));
}

#[test]
fn test_md_files_utf8_no_bom() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for name in &[
        "AGENTS.md",
        "AUTOPILOT.md",
        "ROADMAP.md",
        "SUMMARY.md",
        "README.md",
        "README_EN.md",
        "LICENSE",
    ] {
        let path = root.join(name);
        if path.exists() {
            check_utf8_no_bom(&path);
        }
    }
}
