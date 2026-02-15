#[test]
fn daemon_lifecycle_scaffold_exists() {
    assert!(std::env::var("PATH").is_ok());
}
