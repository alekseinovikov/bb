# bb bash integration scaffold.
# Planned behavior: wrapper function + PROMPT_COMMAND-assisted prefill.

bb() {
  command bb "$@"
}
