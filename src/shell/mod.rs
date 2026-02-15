pub mod bash;
pub mod wrapper;
pub mod zsh;

#[derive(Clone, Copy, Debug)]
pub enum ShellKind {
    Bash,
    Zsh,
}
