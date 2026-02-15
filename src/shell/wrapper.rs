use crate::shell::ShellKind;

pub fn render_output(command: &str, shell: Option<ShellKind>) -> String {
    match shell {
        Some(ShellKind::Bash) => format!("BASH_PREFILL:{command}"),
        Some(ShellKind::Zsh) => format!("ZSH_PREFILL:{command}"),
        None => command.to_string(),
    }
}
