pub trait LlmProvider {
    fn generate_command(&self, prompt: &str) -> anyhow::Result<String>;
}
