#[test]
fn shell_contract_scaffold_exists() {
    assert!(std::env::var("PATH").is_ok());
}
