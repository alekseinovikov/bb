# bb zsh integration scaffold.
# Planned behavior: zle-aware wrapper that places generated command into BUFFER.

bb() {
  command bb "$@"
}
