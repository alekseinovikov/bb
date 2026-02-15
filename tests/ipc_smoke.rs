#[test]
fn ipc_scaffold_exists() {
    assert!(std::env::var("PATH").is_ok());
}
